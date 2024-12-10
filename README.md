# 常微分方程式の数値解法

## 目的

- [Sample_program_ODE](https://github.com/mino2357/Sample_program_ODE) のRustによる改良．
- Rustの勉強．
- 力学系（中心多様体理論，標準形の方法．）の実験と分岐現象観察．
- ポアンカレ写像の数値計算による構成．
- 特異摂動法の実験．

## 指針

- 簡単なものから順に実装していく．
- 個別の問題から取り組む．その関係でRustも簡単な機能から学んでいく．
- はじめのほうの可視化は [gnuplot](http://www.gnuplot.info/) で行う．
  - 標準出力で出力して，gnuplotで可視化するという流れを想定．Rustの数値計算のコードと可視化コードは明確に区別する．

## ここで扱う常微分方程式

以下の $u(t)$ に関する常微分方程式を解く．特に間違えることはないのでスカラーもベクトルも同じ表記とする．場合によって $x, y$ なども使う．

はじめの方は $f(t, u) = f(u)$ で，時間に依存しないものから始める（自励系）． 

$$
\begin{align*}
\frac{du}{dt} = f(t, u) 
\end{align*}
$$

### 扱わないかもしれない微分方程式

遅延微分方程式（DDE）．微分代数方程式（微分に関して陽的に書けないもの，DAE）．

## 参考文献

- [常微分方程式数値解法 Ⅰ, Ⅱ](https://www.maruzen-publishing.co.jp/item/b294285.html) (E.ハイラー (著), 三井 斌友 (翻訳).)
- [非線形の力学系とカオス](https://www.maruzen-publishing.co.jp/item/b294656.html) (Stephen Wiggins.)
- [プログラミング Rust](https://www.oreilly.co.jp/books/9784873119786/) (Jim Blandy、Jason Orendorff、Leonora F. S. Tindall 著、中田 秀基 訳.)
- [Effective Rust ―Rustコードを改善し、エコシステムを最大限に活用するための35項目](https://www.oreilly.co.jp/books/9784814400942/) (David Drysdale 著、中田 秀基 訳.)