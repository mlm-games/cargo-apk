# ndk-examples

Collection of examples showing different parts of the libraries.

## Examples

In order to see logs of the sample apps execute in a console:
```console
$ adb logcat RustStdoutStderr:D '*:S'
```

### hello_world

Prints `hello world` in the console

```console
$ cargo rapk build --example hello_world
```

### jni_audio

Prints output audio devices in the console

```console
$ cargo rapk run --example jni_audio
```
