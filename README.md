# timestamp

@bendem wrote a program like this in C++, so I wanted to write my own version in Rust for funsies.

Default format is %c, but you can change it by passing in any arg.

```sh
% {echo 'Hi'; sleep 2; echo 'Hi again'} | timestamp
[Sat Aug  6 17:04:57 2016] Hi
[Sat Aug  6 17:04:59 2016] Hi again
```

```sh
% {echo 'Hi'; sleep 2; echo 'Hi again'} | timestamp '%S'
[11] Hi
[13] Hi again
```
