;;; Directory Local Variables            -*- no-byte-compile: t -*-
;;; For more information see (info "(emacs) Directory Variables")

((nil . ((compile-command . "rustc --color=always -Cdebuginfo=0 -Cllvm-args=--x86-asm-syntax=intel -Copt-level=1 main.rs --emit asm && cat ./main.s"))))
