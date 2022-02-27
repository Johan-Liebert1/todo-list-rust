const rust = import("../pkg");

rust.then(m => {
    console.log(m);
    m.greet("Pragyan");
});
