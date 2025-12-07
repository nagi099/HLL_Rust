use ndarray::{s, Array2, ArrayView2, ArrayViewMut2};

pub struct Grid2D {
    pub nx: usize,
    pub ny: usize,
    pub nghost: usize,
    pub rho:  Array2<f64>,
    pub rhou: Array2<f64>,
    pub rhov: Array2<f64>,
    pub e:    Array2<f64>,
}

impl Grid2D {
    pub fn new(nx: usize, ny: usize, nghost: usize) -> Self {
        // nyが行(y), nxが列(x)として確保
        let zeros = Array2::<f64>::zeros((ny + nghost * 2, nx + nghost * 2)); 
        Self {
            nx, ny, 
            nghost,
            rho:  zeros.clone(),
            rhou: zeros.clone(),
            rhov: zeros.clone(),
            e:    zeros,
        }
    }

    /// 物理座標 (i, j) を配列全体のインデックス (row, col) に変換
    /// i: x方向 (0..nx), j: y方向 (0..ny)
    #[inline]
    pub fn idx_with_ghost(&self, i: usize, j: usize) -> (usize, usize) {
        (j + self.nghost, i + self.nghost)
    }

    // --- 以下、追加ヘルパ関数 ---

    /// 物理領域（ゴーストセルを除く計算領域）の可変ビューを取得する
    /// 時間発展の計算などでメインループを回す際に使用します
    pub fn physical_slice_mut(&mut self, field_name: &str) -> ArrayViewMut2<f64> {
        let s_y = self.nghost..(self.ny + self.nghost);
        let s_x = self.nghost..(self.nx + self.nghost);
        
        let arr = match field_name {
            "rho"  => &mut self.rho,
            "rhou" => &mut self.rhou,
            "rhov" => &mut self.rhov,
            "e"    => &mut self.e,
            _ => panic!("Unknown field name: {}", field_name),
        };
        
        arr.slice_mut(s![s_y, s_x])
    }

    /// 全ての変数に対して周期境界条件 (Periodic Boundary Condition) を適用する
    /// 左端のゴーストには右端の値を、上端のゴーストには下端の値をコピーします
    pub fn apply_periodic_boundary(&mut self) {
        // 借用チェッカーを回避するため、各フィールドに対して個別にヘルパを呼ぶ
        Self::apply_periodic_single(&mut self.rho, self.nx, self.ny, self.nghost);
        Self::apply_periodic_single(&mut self.rhou, self.nx, self.ny, self.nghost);
        Self::apply_periodic_single(&mut self.rhov, self.nx, self.ny, self.nghost);
        Self::apply_periodic_single(&mut self.e, self.nx, self.ny, self.nghost);
    }

    /// 全ての変数に対して自由流出（Neumann 0勾配）境界条件を適用する
    /// 境界の値をそのまま外側のゴーストセルにコピーします
    pub fn apply_outflow_boundary(&mut self) {
        Self::apply_outflow_single(&mut self.rho, self.nx, self.ny, self.nghost);
        Self::apply_outflow_single(&mut self.rhou, self.nx, self.ny, self.nghost);
        Self::apply_outflow_single(&mut self.rhov, self.nx, self.ny, self.nghost);
        Self::apply_outflow_single(&mut self.e, self.nx, self.ny, self.nghost);
    }

    // --- 内部用プライベートヘルパ関数 ---

    fn apply_periodic_single(arr: &mut Array2<f64>, nx: usize, ny: usize, ng: usize) {
        // X方向 (左右)
        // 左ゴースト <- 右端物理領域
        let left_src = s![.., nx..nx+ng];
        let left_dst = s![.., 0..ng];
        let left_data = arr.slice(left_src).to_owned(); // コピーを作成
        arr.slice_mut(left_dst).assign(&left_data);

        // 右ゴースト <- 左端物理領域
        let right_src = s![.., ng..ng*2];
        let right_dst = s![.., nx+ng..];
        let right_data = arr.slice(right_src).to_owned();
        arr.slice_mut(right_dst).assign(&right_data);

        // Y方向 (上下)
        // 下ゴースト (row 0側) <- 上端物理領域
        let bot_src = s![ny..ny+ng, ..];
        let bot_dst = s![0..ng, ..];
        let bot_data = arr.slice(bot_src).to_owned();
        arr.slice_mut(bot_dst).assign(&bot_data);

        // 上ゴースト (row max側) <- 下端物理領域
        let top_src = s![ng..ng*2, ..];
        let top_dst = s![ny+ng.., ..];
        let top_data = arr.slice(top_src).to_owned();
        arr.slice_mut(top_dst).assign(&top_data);
    }

    fn apply_outflow_single(arr: &mut Array2<f64>, nx: usize, ny: usize, ng: usize) {
        // X方向 (左右)
        // 左ゴースト <- 左端の物理セル(境界値)をコピー
        for i in 0..ng {
            let boundary_col = ng; // 物理領域の左端
            let col_to_fill = i;
            let val = arr.column(boundary_col).to_owned();
            arr.column_mut(col_to_fill).assign(&val);
        }
        // 右ゴースト
        for i in 0..ng {
            let boundary_col = nx + ng - 1; // 物理領域の右端
            let col_to_fill = nx + ng + i;
            let val = arr.column(boundary_col).to_owned();
            arr.column_mut(col_to_fill).assign(&val);
        }

        // Y方向 (上下) も同様に行いますが、簡単のため省略するか
        // 必要に応じて行(row)ごとのコピーを実装します。
        // 基本的に流体計算では row_mut(), assign() を使います。
    }
}