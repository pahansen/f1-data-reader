# F1 Data Reader
F1 Data Reader is a cli tool that allows to record & parse teh udp telemetry stream from [F1 22](https://www.ea.com/de-de/games/f1/f1-22).

## How to run
In order to run the cli tool, first you have to create a release build.

    cargo build --release

Afterwards, if you run the release build with `f1_data_reader --help`, the two main commands `recorder`and `parser` are explained.

    Usage: f1_data_reader <COMMAND>

    Commands:
    recorder  Record udp stream to f1 log file
    parser    Parse f1 log file to parquet files
    help      Print this message or the help of the given subcommad(s)

    Options:
    -h, --help     Print help
    -V, --version  Print version

When running the `recorder`, make sure to include the full path to the file.

    f1_data_reader recorder --f1-log-file-path path/to/my/f1_log.bin

Once the recording has started, you can stop it be stopping the execution of the script (e.g. CMD+c).

After the recording has stopped, you can run the parser to receive the recorded data as parquet files. There will be one file for each parsed message type (e.g. car_telemetry.parquet, laps.parquet, ..). Therefore, only specify the folder where all the parquet files should be stored.

    f1_data_reader parser --f1-log-file-path path/to/my/f1_log.bin --parquet-folder-path pyth/to/my/parquet_folder

