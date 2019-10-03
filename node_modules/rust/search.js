var fs = require('fs');

module.exports = function(options, callback){
  fs.readFile(options.file, function(err, data){
    if (err) throw err;
    data.toString().replace(options.regex, function(ex, search){
      if (options.cleaner) {
        var cleaner = options.cleaner;
        for (var i in cleaner){
          search = search.replace(cleaner[i].clean, cleaner[i].cleanee);
        }
      }
      if (options.trim) search = search.trim();
      callback(search);
    });
  });
};
