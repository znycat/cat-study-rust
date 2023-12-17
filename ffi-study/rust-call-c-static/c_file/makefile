main: main.o cat_custom.o
	gcc -o main main.o cat_custom.o

main_lib: main.o cat_custom.a
	gcc -o main_lib main.o -L. -lcat_custom 

main.o: main.c
	gcc -c main.c -o main.o

cat_custom.o: cat_custom.c
	gcc -c cat_custom.c -o cat_custom.o

cat_custom.a: cat_custom.o
	ar rcs libcat_custom.a cat_custom.o

clean:
	rm -rf main 
	rm -rf main_lib
	rm -rf main.o
	rm -rf cat_custom.o
	rm -rf cat_custom.a

# -c 选项告诉 GCC 只编译源代码文件，生成对应的目标文件（.o 文件），而不进行最终的链接操作
# -o 表示生成的目标文件的名称, 通常在编译或链接命令中使用
# -L. 表示库文件的搜索路径为当前目录, -lcat_custom 表示链接的库名字是cat_custom

# ar rcs 是一个用于创建静态库的命令
# ar 是一个用于创建、修改和提取静态库的工具
# rcs 是 ar 命令的一组选项
# r 选项表示将文件插入到静态库中。如果静态库中已经存在同名的文件，将会被替换。
# c 选项表示创建一个新的静态库文件。如果静态库文件不存在，
#   将会创建一个新的静态库文件；如果静态库文件已经存在，则会清空静态库文件的内容。
# s 选项表示在静态库中创建符号表。符号表用于标识静态库中的函数和变量，
#   以便在链接时进行符号解析。
