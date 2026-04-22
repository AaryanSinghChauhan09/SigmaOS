const fs = require('fs');
const files = fs.readdirSync('web_ui/scripts/modules').filter(f => f.endsWith('.js'));
const modulePaths = files.map(f => '    "scripts/modules/' + f + '"').join(',\n');
console.log(modulePaths);
