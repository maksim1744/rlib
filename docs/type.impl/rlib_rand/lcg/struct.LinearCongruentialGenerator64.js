(function() {var type_impls = {
"rlib_rand":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-LinearCongruentialGenerator64%3CA,+C%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/rlib_rand/lcg.rs.html#6\">source</a><a href=\"#impl-Clone-for-LinearCongruentialGenerator64%3CA,+C%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const A: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a>, const C: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"rlib_rand/lcg/struct.LinearCongruentialGenerator64.html\" title=\"struct rlib_rand::lcg::LinearCongruentialGenerator64\">LinearCongruentialGenerator64</a>&lt;A, C&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rlib_rand/lcg.rs.html#6\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.78.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"rlib_rand/lcg/struct.LinearCongruentialGenerator64.html\" title=\"struct rlib_rand::lcg::LinearCongruentialGenerator64\">LinearCongruentialGenerator64</a>&lt;A, C&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.78.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.78.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.78.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.78.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","rlib_rand::Rng"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-LinearCongruentialGenerator64%3CA,+C%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/rlib_rand/lcg.rs.html#11-29\">source</a><a href=\"#impl-LinearCongruentialGenerator64%3CA,+C%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const A: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a>, const C: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a>&gt; <a class=\"struct\" href=\"rlib_rand/lcg/struct.LinearCongruentialGenerator64.html\" title=\"struct rlib_rand::lcg::LinearCongruentialGenerator64\">LinearCongruentialGenerator64</a>&lt;A, C&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.from_seed\" class=\"method\"><a class=\"src rightside\" href=\"src/rlib_rand/lcg.rs.html#12-14\">source</a><h4 class=\"code-header\">pub const fn <a href=\"rlib_rand/lcg/struct.LinearCongruentialGenerator64.html#tymethod.from_seed\" class=\"fn\">from_seed</a>(seed: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a>) -&gt; Self</h4></section><section id=\"method.from_time\" class=\"method\"><a class=\"src rightside\" href=\"src/rlib_rand/lcg.rs.html#16-23\">source</a><h4 class=\"code-header\">pub fn <a href=\"rlib_rand/lcg/struct.LinearCongruentialGenerator64.html#tymethod.from_time\" class=\"fn\">from_time</a>() -&gt; Self</h4></section><section id=\"method.next_raw\" class=\"method\"><a class=\"src rightside\" href=\"src/rlib_rand/lcg.rs.html#25-28\">source</a><h4 class=\"code-header\">pub fn <a href=\"rlib_rand/lcg/struct.LinearCongruentialGenerator64.html#tymethod.next_raw\" class=\"fn\">next_raw</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a></h4></section></div></details>",0,"rlib_rand::Rng"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Rand-for-LinearCongruentialGenerator64%3CA,+C%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/rlib_rand/lcg.rs.html#31-38\">source</a><a href=\"#impl-Rand-for-LinearCongruentialGenerator64%3CA,+C%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const A: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a>, const C: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a>&gt; <a class=\"trait\" href=\"rlib_rand/trait.Rand.html\" title=\"trait rlib_rand::Rand\">Rand</a> for <a class=\"struct\" href=\"rlib_rand/lcg/struct.LinearCongruentialGenerator64.html\" title=\"struct rlib_rand::lcg::LinearCongruentialGenerator64\">LinearCongruentialGenerator64</a>&lt;A, C&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.next\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rlib_rand/lcg.rs.html#32-37\">source</a><a href=\"#method.next\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rlib_rand/trait.Rand.html#tymethod.next\" class=\"fn\">next</a>&lt;T, R&gt;(&amp;mut self, range: R) -&gt; T<div class=\"where\">where\n    R: <a class=\"trait\" href=\"rlib_rand/randomable/trait.Randomable.html\" title=\"trait rlib_rand::randomable::Randomable\">Randomable</a>&lt;T&gt;,</div></h4></section><section id=\"method.shuffle\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/rlib_rand/mrand.rs.html#8-12\">source</a><a href=\"#method.shuffle\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"rlib_rand/trait.Rand.html#method.shuffle\" class=\"fn\">shuffle</a>&lt;T&gt;(&amp;mut self, v: &amp;mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.slice.html\">[T]</a>)</h4></section></div></details>","Rand","rlib_rand::Rng"],["<section id=\"impl-Copy-for-LinearCongruentialGenerator64%3CA,+C%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/rlib_rand/lcg.rs.html#6\">source</a><a href=\"#impl-Copy-for-LinearCongruentialGenerator64%3CA,+C%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;const A: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a>, const C: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.78.0/std/primitive.u64.html\">u64</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.78.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"rlib_rand/lcg/struct.LinearCongruentialGenerator64.html\" title=\"struct rlib_rand::lcg::LinearCongruentialGenerator64\">LinearCongruentialGenerator64</a>&lt;A, C&gt;</h3></section>","Copy","rlib_rand::Rng"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()