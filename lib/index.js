var addon = require('../native');

console.log(addon.sleep(1));
console.log(addon.sleep(1));
console.log(addon.sleep(1));
console.log(addon.sleep(1));
console.log(addon.sleep(1));
console.log(addon.sleep(1));
console.log(addon.sleep(1));
console.log(addon.sleep(1));
console.log(addon.sleep(1));
console.log(addon.sleep(1));

console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));
console.log(addon.async_sleep(3, (err, i)=>{
    console.log("async sleep " + i);
}));

var stdin = process.openStdin();
stdin.addListener("data", function(d) {
    // note:  d is an object, and when converted to a string it will
    // end with a linefeed.  so we (rather crudely) account for that
    // with toString() and then trim()
    console.log("you entered: [" +
        d.toString().trim() + "]");
});