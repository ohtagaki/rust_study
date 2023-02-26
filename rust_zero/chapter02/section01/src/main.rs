fn main() {
    // 論理演算
    println!("{}",true && false); //論理積（短絡評価）
    println!("{}",false || true); //論理和（短絡評価）
    println!("{}",true & false); //論理積（非短絡評価）
    println!("{}",true | false); //論理和（非短絡評価）
    println!("{}",false ^ true); //排他的論理和
    println!("{}",!true); //論理否定

    // 短絡評価と非短絡評価
    println!("短絡評価");
    println!("{}", a_short_circuit_evaluation() || b_short_circuit_evaluation());

    println!("非短絡評価");
    println!("{}", a_short_circuit_evaluation() | b_short_circuit_evaluation());

    // 加減乗除
    println!("{}",1234 + 567);
    println!("{}",678 - 158);
    println!("{}",56 + 135);
    println!("{}", 240 /18);
    println!("{}", 2023 % 34);

    // 比較
    println!("{}",1234 < 567);
    println!("{}",678 <= 158);
    println!("{}",56 > 135);
    println!("{}", 240 >= 18);
    println!("{}", 2023 == 34);


}

// 短絡評価と非短絡評価
fn a_short_circuit_evaluation() -> bool {
    println!("a_short_circuit_evaluation");
    true
}

fn b_short_circuit_evaluation() -> bool {
    println!("b_short_circuit_evaluation");
    true
}
