# uc 

uc is a parcent encode for command-line tool.

## Options
- mode: parcent **encode** or **decode**
  Positional argument, the default value is ```encode```

- input: String to be converted
  Option positional arugment

## Installing Cargo
In preparation


## Usage
### Encode
```
uc encode 'foo <bar>'
foo%20%3Cbar%3E
```

```
# test.txt
hello
hello
hello
foo <bar>

hello
hello

cat test.txt | uc
hello
hello
hello
foo%20%3Cbar%3E

hello
hello
```

### Decode
```
uc decode foo%20%3Cbar%3E
foo <bar>
```

```
# test.txt
hello
hello
hello
foo%20%3Cbar%3E

hello
hello

cat test.txt | uc decode
hello
hello
hello
foo <bar>

hello
hello
```
