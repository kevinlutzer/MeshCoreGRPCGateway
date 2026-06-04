use tonic::{Response, Status};

use crate::meshcore_proto::ResetResponse;

pub async fn reset() -> Result<Response<ResetResponse>, Status> {
    return Err(Status::unimplemented("Reset is not implemented yet"));
}
