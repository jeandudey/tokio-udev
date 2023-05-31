;;; SPDX-FileCopyrightText: © 2023 Jean-Pierre De Jesus DIAZ <me@jeandudey.tech>
;;; SPDX-License-Identifier: GPL-3.0-or-later

(use-modules (gnu packages)
             (gnu packages crates-io)
             (gnu packages linux)
             (gnu packages pkg-config)
             (guix build-system cargo)
             (guix download)
             (guix gexp)
             (guix git-download)
             ((guix licenses) #:prefix license:)
             (guix packages))

;;; Commentary:
;;
;; To build and install, run:
;;
;;   guix package -f guix.scm
;;
;; To build it, but not install it, run:
;;
;;   guix build -f guix.scm
;;
;; To use as the basis for a development environment, run:
;;
;;   guix environment -l guix.scm
;;
;;; Code:

(define-public rust-libudev-sys-0.1
  (package
    (name "rust-libudev-sys")
    (version "0.1.4")
    (source (origin
              (method url-fetch)
              (uri (crate-uri "libudev-sys" version))
              (file-name (string-append name "-" version ".tar.gz"))
              (sha256
                (base32
                  "09236fdzlx9l0dlrsc6xx21v5x8flpfm3d5rjq9jr5ivlas6k11w"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs
       (("rust-libc" ,rust-libc-0.2)
        ("rust-pkg-config" ,rust-pkg-config-0.3))))
    (native-inputs (list pkg-config))
    (inputs (list eudev))
    (home-page "https://github.com/dcuddeback/libudev-sys")
    (synopsis "FFI bindings to libudev")
    (description "FFI bindings to libudev")
    (license license:expat)))

(define-public rust-udev-0.7
  (package
    (name "rust-udev")
    (version "0.7.0")
    (source (origin
              (method url-fetch)
              (uri (crate-uri "udev" version))
              (file-name (string-append name "-" version ".tar.gz"))
              (sha256
                (base32
                  "06hr927z0fdn7ay0p817b9x19i5fagmpmvz95yhl4d1pf3bbpgaf"))))
    (build-system cargo-build-system)
    (arguments
      `(#:cargo-inputs
        (("rust-libc" ,rust-libc-0.2)
         ("rust-libudev-sys" ,rust-libudev-sys-0.1)
         ("rust-mio" ,rust-mio-0.8)
         ("rust-mio" ,rust-mio-0.7)
         ("rust-mio" ,rust-mio-0.6)
         ("rust-pkg-config" ,rust-pkg-config-0.3))))
    (native-inputs (list pkg-config))
    (inputs (list eudev))
    (home-page "https://github.com/Smithay/udev-rs")
    (synopsis "libudev bindings for Rust")
    (description "libudev bindings for Rust")
    (license license:expat)))

(define-public rust-tokio-udev-0.9.0
  (package
    (name "tokio-udev")
    (version "0.9.0")
    (source (local-file "tokio-udev"
                        "rust-tokio-udev"
                        #:recursive? #t
                        #:select? (git-predicate "tokio-udev")))
    (build-system cargo-build-system)
    (arguments
      `(#:cargo-inputs
         (("rust-futures-core" ,rust-futures-core-0.3)
          ("rust-futures-util" ,rust-futures-util-0.3)
          ("rust-tokio" ,rust-tokio-1)
          ("rust-udev" ,rust-udev-0.7))))
    (native-inputs (list pkg-config))
    (inputs (list eudev))
    (home-page "https://github.com/jeandudey/tokio-udev")
    (synopsis "Monitor udev/eudev events asynchronously")
    (description "This Rust library provides abstractions to wait for
udev/eudev events asycnchronously using the Tokio runtime.")
    (license (list license:expat license:asl2.0))))

rust-tokio-udev-0.8
