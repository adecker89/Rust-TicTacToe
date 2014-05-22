#include <stdio.h>

extern void* init_board(unsigned int m, unsigned int n, unsigned int k);
extern void destroy_board(void* board);
extern unsigned char make_move(void* board, unsigned int x, unsigned int y, unsigned int *xout, unsigned int *yout);
extern void print_board(void* board);

static const unsigned int INVALID_MOVE = 0;
static const unsigned int PLAYER_WINS= 1;
static const unsigned int AI_WINS = 2;
static const unsigned int CATS_GAME = 3;
static const unsigned int IN_PROGRESS = 4;

int main() {
	void* board = init_board(3,3,3);
	unsigned int x, y, xout, yout;
	char line[256];

	while(1) {
		print_board(board);
		printf("Enter a move: x,y: ");
		if (fgets(line, sizeof(line), stdin)) {
		    if (sscanf(line, "%d,%d", &x, &y) == 2) {
		        unsigned char status = make_move(board,x,y,&xout,&yout);

		        if(status == INVALID_MOVE) {
		        	printf("Invalid move\n");
		        	continue;
		        }

		        if(status != IN_PROGRESS) {
		        	switch(status) {
	        		case PLAYER_WINS:
		        		printf("Player Wins!\n");
		        		break;
        			case AI_WINS:
		        		printf("Ai Wins!\n");
		        		break;
	        		case CATS_GAME:
		        		printf("Cats Game\n");
		        		break;
		        	}
		        	break;
		        } else {
		        	 printf("Ai places O at %d,%d\n",xout,yout);
		        }
		    } else {
		    	printf("Invalid input\n"); 
		    }
		}
	}
	print_board(board);
	destroy_board(board);
	return 0;
}
