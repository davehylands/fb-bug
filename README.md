This project is trying to replicate a biug we're seeing where
i64's get written into the flatbuffer with an alignment that's
not a multiple of 8.

To build:
```
make
```
That will run flatc to build the generated rust code and then 
run the sample app.

You can ignore all of the warnings from the rust compiler that
come from fb_generated.rs.

If you see the `*** timestamp offset not 8-byte aligned ***` message
printed then you've managed to reproduce the bug.

So far, I've not been able to reproduce the problem.
