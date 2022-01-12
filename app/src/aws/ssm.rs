use aws_sdk_ssm::Client;
use std::env;

pub async fn load_env() {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let path = env::var("SSM_PARAMETER").expect("need set ssm parameter");

    match client
        .get_parameter()
        .name(path)
        .with_decryption(true)
        .send()
        .await
    {
        Ok(response) => {
            let values = response
                .parameter
                .unwrap()
                .value
                .unwrap()
                .split("\n")
                .into_iter()
                .map(String::from)
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>();
            values.into_iter().for_each(|v| {
                let keyval = v
                    .split("=")
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>();
                if keyval.len() == 2 {
                    println!("{}={}", &keyval[0], &keyval[1]);
                    env::set_var(&keyval[0], &keyval[1]);
                }
            });
        }
        Err(error) => {
            panic!("got an error getting the ssm parameter: {}", error)
        }
    }
}
