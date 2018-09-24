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

let k = "t = " + Date.now();
stub.put({ key: k, value: "placeholder" }, function(err, response) {
  if (err) {
    console.error("[ERROR] ", err);
  } else {
    console.log("put: ", response);
  }
});

stub.get({ key: k }, function(err, response) {
  if (err) {
    console.error("[ERROR] ", err);
  } else {
    console.log("got: ", response);
  }
});
