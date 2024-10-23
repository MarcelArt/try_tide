// #[macro_export]
// macro_rules! send_json_response {
//     ($obj:expr, $status_code:expr) => {
//         let body = serde_json::to_string(&$obj)?;

//         let res = Response::builder($status_code)
//             .content_type(JSON)
//             .body(body)
//             .build();
//         Ok(res)
//     };
// }

// #[macro_export]
// macro_rules! send_json_response_breaking {
//     ($obj:expr, $status_code:expr) => {
//         let body = serde_json::to_string(&$obj)?;

//         let res = Response::builder($status_code)
//             .content_type(JSON)
//             .body(body)
//             .build();
//         return Ok(res)
//     };
// }