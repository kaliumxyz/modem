# modem ♬

welcome to the modem waveform definition language.
modem stands for modulation / demodulation this is a modulation oriented forth like language
this means its stack based, has `words` as functions, and most important operations are intended to modify.

Modem language exists for the purpose of making music. The code you write gets ran for every single frame of a stream of sound.

modem is inspired by ibniz, I recommend you check it out, its a cool project


TODO
[ ] add sample rate
    [ ] decide on a default sample rate
    [ ] or use durations instead? maybe make it modal?
[ ] multiple sound streams
[ ] link to ibniz with shoutout
[ ] toturial via example files
[ ] documentation
[ ] settle on a syntax
[ ] parsing
    [ ] numbers
[ ] words
    [ ] scoped state
[ ] figure out this parser combinator thing
[ ] make visualizer (seperate project?)


``` forth

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

``` forth
// welcome to the modem waveform definition language
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
// play sine(time program has been playing) from all plugged in speakers
devices loop t sin swap play end
// play 100hz * times program is playing sine from default
default loop t 100 * sin swap play end

// ideas
// perhaps it should have one function per stream somehow to be cleaner?
// stack based languages are a pain to work with for most people, either fix it in syntax by adding variables or add a visualizer oslt
// Do state scoped to functions to do variables.
// e.g. : my_var $s ; : $my_var . ; oslt, get from special scoped stacks
```
