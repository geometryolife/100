# Build a stub for a REST web service

[存根或桩（Stub / Method Stub ）](https://zh.wikipedia.org/wiki/%E6%A1%A9_(%E8%AE%A1%E7%AE%97%E6%9C%BA))是指用来替换一部分功能的程序段。
桩程序可以用来模拟已有程序的行为（比如一个远端机器的过程）或是对将要开发的代码的一种临时替代。
因此，打桩技术在程序移植、分布式计算、通用软件开发和测试中用处很大。


## Input

Test it with the following commands:

```shell
curl -X DELETE http://localhost:8080/datafile.txt
curl -X GET http://localhost:8080/datafile.txt
curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
curl -X POST http://localhost:8080/data -d "File contents."
curl -X GET http://localhost:8080/a/b
```

After running the second command, the client should have printed: `Contents of the file.`

```shell
joe@MX:~$ curl -X DELETE http://localhost:8080/datafile.txt
joe@MX:~$ curl -X GET http://localhost:8080/datafile.txt
Contents of the file.
joe@MX:~$ curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
joe@MX:~$ curl -X POST http://localhost:8080/data -d "File contents."
data17.txtjoe@MX:~$ curl -X GET http://localhost:8080/a/b
```

## Output

After running all five commands, the server should have printed:

```shell
Listening at address 127.0.0.1:8080 ...
Deleting file "datafile.txt" ... Deleted file "datafile.txt"
Downloading file "datafile.txt" ... Downloaded file "datafile.txt"
Uploading file "datafile.txt" ... Uploaded file "datafile.txt"
Uploading file "data*.txt" ... Uploaded file "data17.txt"
Invalid URI: "/a/b"
```
