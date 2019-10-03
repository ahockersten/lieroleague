var fs = require('fs');

module.exports = function(params, callback){
  fs.readFile(params.file, function(err, blob){
    if (err) throw err;
    blob.toString().replace(params.regex, function(r, c){
      fs.writeFile(params.file, blob.toString().replace(r, params.value), callback);
    });
  });
};
