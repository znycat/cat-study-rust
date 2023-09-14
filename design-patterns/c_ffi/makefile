# LDFLAGS := -Wl,--gc-sections -lpthread -ldl

exe: main.o ../target/debug/libdesign_patterns.a
	$(CC) -o $@ $^ 

main.o: main.c | exe
	$(CC) -o $@ -c $<

clean: 
	rm -rf exe *.o
