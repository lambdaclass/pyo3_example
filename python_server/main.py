import ffi

outer = ffi.Outer()
printer = ffi.PyPrinter()

printer.run(outer)

