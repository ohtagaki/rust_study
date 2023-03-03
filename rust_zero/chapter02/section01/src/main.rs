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

    // タプル型
    let _tuple: (i32, char) = (200, 'c');
    println!("tuple {:?}", _tuple);
    // 要素を参照（インデックス）
    println!("tuple.0 {}", _tuple.0);
    println!("tuple.1 {}", _tuple.1);
    // 分解
    let (i1, c1) = _tuple;
    println!("i1 {}", i1);
    println!("c1 {}", c1);

    // 関数ポインタ型
    do_it(add, 23, 3);
    do_it(mul, 23, 3);

    // ユーザ定義型
    enum DoW {
        // variant
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday
    }
    let day = DoW::Friday;

    enum Storage {
        HDD {
            size: u32,
            rpm: u32
        },
        SSD(u32),
    }
    let hdd_sample = Storage::HDD { size: 512, rpm: 9600 };
    let sdd_sample = Storage::SSD(512);

    // 構造体
    struct PCspec {
        cpuSpec: u16,
        memorySpec: u32,
        storage: Storage,
    }
    // PCSpec型の値
    let spec = PCspec {
        cpuSpec: 8,
        memorySpec: 16,
        storage: Storage::SSD(1024),
    };
    println!("{}", spec.cpuSpec);

    // ジェネリクス
    // リンクリストを表すジェリック型
    enum List<T> {
        // T：ジェネリック型の引数で、ここに型や定数を渡すことができる
        Node { data: T, next: Box<List<T>> },
        Nil,
    }

    let generic_sample1 = List::<u32>::Nil;
    let generic_sample2 = List::<u32>::Node {
        data: 8, 
        next: Box::<List<u32>>::new(generic_sample1) };
    let generic_sample3 = List::Node {
        data: 32, 
        next: Box::new(generic_sample2) };

    make_pair::<u8, bool>(40, false);
    make_pair(20, true);

    // 定数を受け取るジェネリック型の例
    struct Buffer<const S: usize> {
        buf: [u16; S],
    }
    let buf = Buffer::<64> {buf: [0; 64]};

    // Option型とResult型
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // 型変換
    // asを用いた変換
    let n :i32 = 100;
    let m: i64 = n as i64;
    println!("{}", m);

    //fromまたはintoを用いた型変換
    let s_from = String::from("apple"); // &strからString型への変換
    let s_into: String = "orange".into(); // &strからString型への変換
    println!("{}", s_from);
    println!("{}", s_into);

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

// ユニット型の返り値
fn func_unit1() -> () {} // ユニット型を返す
fn func_unit2() {} // -> ()は省略可能

// 関数ポインタの例
fn do_it(f: fn(u32,u32) -> u32, a:u32, b:u32) {
    println!("{}", f(a, b));
}

fn add (a:u32, b:u32) -> u32 {
    return a + b;
}

fn mul (a:u32, b:u32) -> u32 {
    return a * b;
}

// ジェネリック関数の例
fn make_pair<T1, T2>(a: T1, b:T2) -> (T1, T2) {
    return (a, b);
}
