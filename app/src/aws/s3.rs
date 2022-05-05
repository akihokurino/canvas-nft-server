use crate::AppResult;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::Client;
use bytes::Bytes;
use http::Uri;
use std::time::Duration;

pub async fn upload_object(
    bucket: String,
    key: String,
    data: Bytes,
    content_type: String,
) -> AppResult<String> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let body = ByteStream::from(data);
    client
        .put_object()
        .bucket(&bucket)
        .key(&key)
        .content_type(content_type)
        .body(body)
        .send()
        .await?;
    Ok(format!(
        "https://{}.s3.ap-northeast-1.amazonaws.com/{}",
        bucket, key
    ))
}

pub async fn download_object(bucket: String, key: String) -> AppResult<Bytes> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let resp = client.get_object().bucket(&bucket).key(&key).send().await?;

    let data = resp.body.collect().await;
    Ok(data.unwrap().into_bytes())
}

pub async fn pre_sign_for_upload(bucket: String, key: String) -> AppResult<Uri> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let expires_in = Duration::from_secs(60 * 60);

    let pre_signed = client
        .put_object()
        .bucket(&bucket)
        .key(&key)
        .presigned(PresigningConfig::expires_in(expires_in).unwrap())
        .await?;

    Ok(pre_signed.uri().to_owned())
}

pub async fn pre_sign_for_get(bucket: String, key: String) -> AppResult<Uri> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let expires_in = Duration::from_secs(60 * 60);

    let pre_signed = client
        .get_object()
        .bucket(&bucket)
        .key(&key)
        .presigned(PresigningConfig::expires_in(expires_in).unwrap())
        .await?;

    Ok(pre_signed.uri().to_owned())
}
