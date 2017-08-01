pub fn raindrops(n: i32) -> String {
    let mut s = "".to_string();
    let has3 = (n % 3) == 0;
    let has5 = (n % 5) == 0;
    let has7 = (n % 7) == 0;
    let hasnone = !(has3 || has5 || has7);

    if has3 {
        s = s + "Pling";
    }
    if has5 {
        s = s + "Plang";
    }
    if has7 {
        s = s + "Plong";
    }
    if hasnone {
        s = n.to_string();
    }
    s
}
