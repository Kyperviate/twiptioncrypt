mod twiption;

fn main() {
    let pt = String::from("THENARWHALBACONSATMIDNIGHT");
    let key = String::from("REDDIT");
    let ct = twiption::encrypt(&pt, &key);
    let decrypted = twiption::decrypt(&ct, &key);
    println!("pt:        {}", pt);
    println!("key:       {}", key);
    println!("ct:        {}", ct);
    println!("decrypted: {}", decrypted);
}
