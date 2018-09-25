var PROTO_PATH = __dirname + '/../src/proto/kv.proto';
var grpc = require('grpc');
var protoLoader = require('@grpc/proto-loader');
// Suggested options for similarity to existing grpc.load behavior
var packageDefinition = protoLoader.loadSync(
    PROTO_PATH,
    {keepCase: true,
     longs: String,
     enums: String,
     defaults: true,
     oneofs: true
    });
var kv = grpc.loadPackageDefinition(packageDefinition);

var stub = new kv.KvService('localhost:50051', grpc.credentials.createInsecure());

async function put(k, v) {
  return new Promise((resolve, reject) => {
    stub.put({ item: {key: k, value: v} }, function(err, response) {
      if (err) {
        console.error("[ERROR] ", err);
        reject(err);
      } else {
        console.log("put: ", response);
        resolve(response);
      }
    });
  });
}

async function scan(prefix) {

  return new Promise((resolve, reject) => {
    stub.scan({ prefix: prefix }, function(err, response) {
      if (err) {
        console.error("[ERROR] ", err);
        reject(err);
      } else {
        console.log("scan: ", response);
        resolve(response);
      }
    });
  });
}

async function main() {
  await put("foo/0", "asdf");
  await put("foo/1", "pqrs");
  await put("foo/" + Date.now(), "last");
  await put("xyz", "tuvw");

  let call = stub.scan({ prefix: "foo" });
  call.on('data', function(item) {
    console.log("received ", item);
  });
  call.on('end', function() {
    console.log("end");
  });
  call.on('error', function(e) {
    console.log("error: ", e);
  });
  call.on('status', function(status) {
    console.log("status: ", status);
  });
}


main();
