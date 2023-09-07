package cgo

/*
#cgo LDFLAGS: -L${SRCDIR}/.. -lfilcrypto
#cgo LDFLAGS: /usr/lib/libpc2.a
#cgo pkg-config: ${SRCDIR}/../filcrypto.pc
#include "../filcrypto.h"
#include <stdlib.h>
*/
import "C"
