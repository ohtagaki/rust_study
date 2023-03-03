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

    // 浮動小数点型
    // 加減乗除
    println!("{}", 1.52 + 0.435);
    println!("{}", 3.6 - 0.96);
    println!("{}", 5.6 * 14.6);
    println!("{}", 58.0 / 41.5);
    println!("{}", 73.0 % 23.4);
    // 比較
    println!("{}", 1.45 < 0.53);
    println!("{}", 2.3 <= 1.2);
    println!("{}", 5.6 > 14.2);
    println!("{}", 58.2 >= 41.5);
    println!("{}", 73.5 == 23.4);

    // ビット演算
    let player: u16 = 
        1 | 
        (1 << 1) |
        (3 << 2);

    if player & 1 != 0 {
        println!("毒状態");
    }

    if player & (1 << 1)  != 0 {
        println!("攻撃力アップ状態");
    }
    println!("playerの値 = {player}");
    let hp = (player & 0xfffc) >> 2; // ビットマスクを用いた計算
    println!("残り体力 = {hp}");

    // 参照と破壊的代入の例
    let mut n: u64 = 100; // nは破壊的代入可能
    let a: &u64 = &n; // aにnの不変参照を代入
    // aを参照外ししした値とアドレスを表示
    println!("*a = {}, address = {:p}", *a, a);

    let b: &mut u64 = &mut n; // bにnの可変参照を代入
    *b = 300;
    println!("*n = {n}");

    // 配列型とスライス
    let arr: [u32; 4] = [2, 4, 6, 8]; // 配列を定義
    println!("{}, {}, {}, {}", arr[0],arr[1],arr[2],arr[3]);
    let slic: &[u32] = &arr[2..4]; //スライスを取得
    println!("{:?}", slic);

    // 文字と文字列の型
    let a: &str = "hello"; // 文字列スライス
    let mut b: String = a.to_string(); // Stringに変換
    b += " , world!  ";
    let c: &str = b.trim(); // 前後の空白文字を取り除いたスライスを取得
    println!("{c}");


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
