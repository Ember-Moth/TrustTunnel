use crate::http_codec::HttpCodec;
use crate::{log_id, log_utils};


pub(crate) async fn listen(mut codec: Box<dyn HttpCodec>, log_id: log_utils::IdChain<u64>) {
    match codec.listen().await {
        Ok(Some(x)) => {
            log_id!(trace, log_id, "Received request: {:?}", x.request().request());
            if let Err(e) = x.split().1.send_ok_response(true) {
                log_id!(debug, log_id, "Failed to send ping response: {}", e);
            }
        }
        Ok(None) => log_id!(debug, log_id, "Connection closed before any request"),
        Err(e) => log_id!(debug, log_id, "Session error: {}", e),
    }

    if let Err(e) = codec.graceful_shutdown().await {
        log_id!(debug, log_id, "Failed to shut down session: {}", e);
    }
}
