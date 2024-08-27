let scaffold_s3 = variable::get("s3_bucket");
let scaffold_sqs = variable::get("sqs_queue");

if !scaffold_s3 {
    file::delete("src/aws/s3.rs");
}

if !scaffold_sqs {
    file::delete("src/aws/sqs.rs");
    file::delete("src/bin/handlers/queues");
}
