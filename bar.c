#include <stdio.h>

extern void* init_board(unsigned int m, unsigned int n, unsigned int k);
extern unsigned char make_move(void* board, unsigned int x, unsigned int y, unsigned int *xout, unsigned int *yout);
extern void print_board(void* board);

int main() {
	void* board = init_board(3,3,3);
	unsigned int x;
	unsigned int y;
	unsigned char is_valid = make_move(board,0,0,&x,&y);
	print_board(board);
	printf("%d\n",is_valid);
	printf("(%d,%d)\n",x,y);
	
	x=0;y=0;
	is_valid = make_move(board,4,1,&x,&y);
	print_board(board);
	printf("%d\n",is_valid);
	printf("(%d,%d)\n",x,y);

	x=0;y=0;
	is_valid = make_move(board,0,2,&x,&y);
	print_board(board);
	printf("%d\n",is_valid);
	printf("(%d,%d)\n",x,y);
	
	x=0;y=0;
	is_valid = make_move(board,1,2,&x,&y);
	print_board(board);
	printf("%d\n",is_valid);
	printf("(%d,%d)\n",x,y);
	
	x=0;y=0;
	is_valid = make_move(board,2,2,&x,&y);
	print_board(board);
	printf("%d\n",is_valid);
	printf("(%d,%d)\n",x,y);
	return 0;
}
