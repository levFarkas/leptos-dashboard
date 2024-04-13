// use aws_config::BehaviorVersion;
// use aws_sdk_dynamodb::Client;

// pub async fn aws_credential() {
//     let config = aws_config::defaults(BehaviorVersion::latest())
//         .region("eu-west-1")
//         .load()
//         .await;
//     let client = Client::new(&config);
//     let paginator = client.list_tables().into_paginator().items().send();
//     let table_names = paginator.collect::<Result<Vec<_>, _>>().await;
//     println!("{:?}", table_names)
// }
