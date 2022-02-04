```
// welcome to the modem language
// modem stands for modulation / demodulation
// this is a modulation based forth like stack
// based programming language for the purpose
// of making music. Its functional and blahblah

// in forth you have no functions, instead you
// use words.
pi sin // will result in 1 being on the stack

// much like forth modem uses words which you can
// define. The arguments are implicit in forth.
: name_of_word operation operation ;
// if statement
: name_of_word operation_returning_true if operation else other_operation ;
// ret for early return
: name_of_word operation_returning_true if return operation else other_operation end other_operation ;
// list audio devices, break if stack is empty? also end loops with break?
devices loop print end
// play 400hrtz sine from all plugged in speakers
devices loop t sin swap play end
// play 200htz sine from default
default loop t sin swap play end

// ideas
// perhaps it should have one function per stream somehow to be cleaner?
// stack based languages are a pain to work with for most people, either fix it in syntax by adding variables or add a visualizer oslt

```
