use salvo:: prelude::*;
use nanoid::nanoid;
use sub_conver::*;

#[handler]
pub async fn new(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let req = req.parse_json::<MyRequest>().await.unwrap();
    let json = serde_json::to_string(&req).unwrap();
    let urlid = nanoid!();
    let state = depot.obtain::<DBPool>().unwrap().lock().await;
    sqlx::query!(
        r#"INSERT INTO main (urlid, json) VALUES (?, ?)"#,
        urlid,
        json
    ).execute(&state.db_pool).await.unwrap();
    let domain = state.domain.clone();
    let port = state.port.clone();
    println!("new sub! http://{}:{}/{}", domain, port, urlid);
    res.render(Text::Plain(format!("http://{}:{}/{}", domain, port, urlid)));
}

#[handler]
pub async fn get(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let urlid = req.param::<String>("urlid").unwrap();
    let db_pool = depot.obtain::<DBPool>().unwrap().lock().await.db_pool.clone();
    let row = sqlx::query!(
        r#"SELECT json FROM main WHERE urlid = ?"#,
        urlid
    ).fetch_one(&db_pool).await.unwrap();
    let json: MyRequest = serde_json::from_str(&row.json).unwrap();
    let urlid = json.url;
    let wireguard = json.wireguard;
    let wireguard_proxy = Proxy {
        name: wireguard.name.clone(),
        server: wireguard.server_ip,
        port: wireguard.server_port.parse::<u16>().unwrap(),
        proxy_type: "wireguard".to_string(),
        uuid: None,
        alterid: None,
        cipher: None,
        tls: None,
        network: None,
        ws_opts: None,
        password: None,
        sni: None,
        skip_cert_verify: None,
        private_key: Some(wireguard.private_key),
        public_key: Some(wireguard.public_key),
        ip: Some(wireguard.client_ip),
        allowed_ips: Some(
            wireguard.allowed_ips.split("\n").map(|x|x.to_string()).collect()
        ),
    };
    let wireguard_group = ProxyGroup {
        name: "🤖 虚拟组网".to_string(),
        group_type: "select".to_string(),
        proxies: vec![wireguard.name.clone(),"DIRECT".to_string(),"REJECT".to_string()],
        url: None,
        interval: None,
        tolerance: None,
        strategy: None,
    };
    let config_string = reqwest::get(&urlid).await.unwrap().text().await.unwrap();
    let mut config: Config = serde_yaml::from_str(&config_string).unwrap();
    config.proxy_groups.push(wireguard_group);
    for allow_ip in wireguard_proxy.allowed_ips.clone().unwrap().iter() {
        config.rules.insert(0,format!("IP-CIDR,{},🤖 虚拟组网,no-resolve", allow_ip));
    }
    config.proxies.push(wireguard_proxy);
    let mut config_yaml = serde_yaml::to_string(&config).unwrap();
    config_yaml = config_yaml.replace("（", "(");
    config_yaml = config_yaml.replace("）", ")");
    // println!("yaml: {:?}", config);
    // println!("url :{:?}",urlid);
    res.render(Text::Plain(config_yaml))
}

#[handler]
pub async fn index(req: &mut Request, res: &mut Response) {
    res.send_file("index.html", req.headers()).await;
}