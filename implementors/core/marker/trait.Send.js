(function() {var implementors = {};
implementors["rust_examples"] = [{"text":"impl&lt;'a, K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"rust_examples/adts/enum.Tree.html\" title=\"enum rust_examples::adts::Tree\">Tree</a>&lt;'a, K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rust_examples::adts::Tree"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/adts/struct.SelfReferentialStructureTest.html\" title=\"struct rust_examples::adts::SelfReferentialStructureTest\">SelfReferentialStructureTest</a>","synthetic":true,"types":["rust_examples::adts::SelfReferentialStructureTest"]},{"text":"impl&lt;'id&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/brands/struct.InvariantLifetime.html\" title=\"struct rust_examples::brands::InvariantLifetime\">InvariantLifetime</a>&lt;'id&gt;","synthetic":true,"types":["rust_examples::brands::InvariantLifetime"]},{"text":"impl&lt;'id&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/brands/struct.BrandedIndex.html\" title=\"struct rust_examples::brands::BrandedIndex\">BrandedIndex</a>&lt;'id&gt;","synthetic":true,"types":["rust_examples::brands::BrandedIndex"]},{"text":"impl&lt;'id, T&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/brands/struct.BrandedVec.html\" title=\"struct rust_examples::brands::BrandedVec\">BrandedVec</a>&lt;'id, T&gt;","synthetic":true,"types":["rust_examples::brands::BrandedVec"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/dependent/struct.Zero.html\" title=\"struct rust_examples::dependent::Zero\">Zero</a>","synthetic":true,"types":["rust_examples::dependent::Zero"]},{"text":"impl&lt;N&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/dependent/struct.Succ.html\" title=\"struct rust_examples::dependent::Succ\">Succ</a>&lt;N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["rust_examples::dependent::Succ"]},{"text":"impl&lt;N, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/dependent/struct.Vector.html\" title=\"struct rust_examples::dependent::Vector\">Vector</a>&lt;N, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["rust_examples::dependent::Vector"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/dependent/struct.HNil.html\" title=\"struct rust_examples::dependent::HNil\">HNil</a>","synthetic":true,"types":["rust_examples::dependent::HNil"]},{"text":"impl&lt;H, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/dependent/struct.HCons.html\" title=\"struct rust_examples::dependent::HCons\">HCons</a>&lt;H, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["rust_examples::dependent::HCons"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/dispatch/struct.Quadratic.html\" title=\"struct rust_examples::dispatch::Quadratic\">Quadratic</a>","synthetic":true,"types":["rust_examples::dispatch::Quadratic"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"rust_examples/dispatch/enum.Trigonometric.html\" title=\"enum rust_examples::dispatch::Trigonometric\">Trigonometric</a>","synthetic":true,"types":["rust_examples::dispatch::Trigonometric"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/dispatch/struct.CannotMonomorphizeDifferentiableInVecTest.html\" title=\"struct rust_examples::dispatch::CannotMonomorphizeDifferentiableInVecTest\">CannotMonomorphizeDifferentiableInVecTest</a>","synthetic":true,"types":["rust_examples::dispatch::CannotMonomorphizeDifferentiableInVecTest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/memory/struct.Point2D.html\" title=\"struct rust_examples::memory::Point2D\">Point2D</a>","synthetic":true,"types":["rust_examples::memory::Point2D"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/memory/struct.RGBColor.html\" title=\"struct rust_examples::memory::RGBColor\">RGBColor</a>","synthetic":true,"types":["rust_examples::memory::RGBColor"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/memory/struct.Palette.html\" title=\"struct rust_examples::memory::Palette\">Palette</a>&lt;'a&gt;","synthetic":true,"types":["rust_examples::memory::Palette"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/memory/struct.DoubleFeeTest.html\" title=\"struct rust_examples::memory::DoubleFeeTest\">DoubleFeeTest</a>","synthetic":true,"types":["rust_examples::memory::DoubleFeeTest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/memory/struct.UseAfterFreeTest.html\" title=\"struct rust_examples::memory::UseAfterFreeTest\">UseAfterFreeTest</a>","synthetic":true,"types":["rust_examples::memory::UseAfterFreeTest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/memory/struct.AliasingXorMutabilityTest.html\" title=\"struct rust_examples::memory::AliasingXorMutabilityTest\">AliasingXorMutabilityTest</a>","synthetic":true,"types":["rust_examples::memory::AliasingXorMutabilityTest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/memory/struct.ExclusiveOwnershipTest.html\" title=\"struct rust_examples::memory::ExclusiveOwnershipTest\">ExclusiveOwnershipTest</a>","synthetic":true,"types":["rust_examples::memory::ExclusiveOwnershipTest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"rust_examples/orphan/model/enum.Entity.html\" title=\"enum rust_examples::orphan::model::Entity\">Entity</a>","synthetic":true,"types":["rust_examples::orphan::model::Entity"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/orphan/struct.OrphanInstanceTest.html\" title=\"struct rust_examples::orphan::OrphanInstanceTest\">OrphanInstanceTest</a>","synthetic":true,"types":["rust_examples::orphan::OrphanInstanceTest"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/rc/struct.Data.html\" title=\"struct rust_examples::rc::Data\">Data</a>","synthetic":true,"types":["rust_examples::rc::Data"]},{"text":"impl&lt;'a&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/rc/struct.Container.html\" title=\"struct rust_examples::rc::Container\">Container</a>&lt;'a&gt;","synthetic":true,"types":["rust_examples::rc::Container"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/rsqrt/struct.PositiveFloat.html\" title=\"struct rust_examples::rsqrt::PositiveFloat\">PositiveFloat</a>","synthetic":true,"types":["rust_examples::rsqrt::PositiveFloat"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/rsqrt/struct.Float.html\" title=\"struct rust_examples::rsqrt::Float\">Float</a>","synthetic":true,"types":["rust_examples::rsqrt::Float"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/typing/struct.Positive.html\" title=\"struct rust_examples::typing::Positive\">Positive</a>","synthetic":true,"types":["rust_examples::typing::Positive"]},{"text":"impl&lt;T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/typing/struct.TopTypeExample.html\" title=\"struct rust_examples::typing::TopTypeExample\">TopTypeExample</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["rust_examples::typing::TopTypeExample"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/typing/struct.BottomTypeExample.html\" title=\"struct rust_examples::typing::BottomTypeExample\">BottomTypeExample</a>","synthetic":true,"types":["rust_examples::typing::BottomTypeExample"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.59.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"rust_examples/typing/struct.UnitTypeExample.html\" title=\"struct rust_examples::typing::UnitTypeExample\">UnitTypeExample</a>","synthetic":true,"types":["rust_examples::typing::UnitTypeExample"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()