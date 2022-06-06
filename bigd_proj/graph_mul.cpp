#include <iostream>
#include <stdlib.h>
#include <math.h>
#include <fstream>
#include <time.h>


using namespace std;

int main(int argc, char **argv){

    if(argc != 3){
        cout << "You need to give parameters nodes of initial graph and multiplication factor\n";
        return 0;
    }

    int nodes = atoi(argv[1]); // arihtmos komvwn arxikou grafhmatos
    int mul = atoi(argv[2]); // paragontas pollaplasiasmou

    ifstream MyInput("random_graph.txt");
    ofstream TempFile("temp.txt");
    
    clock_t tim;
    tim = clock();

    int a,b;

    while (MyInput >> a >> b){
        for(int i = 0; i<mul; i++){
            int tempa = nodes*i+a;
            for(int j = 0; j<mul; j++){
                int tempb = nodes*j+b;
                if(tempa<tempb)
                    TempFile << to_string(tempa) << " " << to_string(tempb) << "\n";
                else
                    TempFile << to_string(tempb) << " " << to_string(tempa) << "\n";

            }
        }
    }

    MyInput.close();
    TempFile.close();

    ifstream TempInp("temp.txt");
    ofstream MyFile("random_graph.txt");

    while (TempInp >> a >> b)
        MyFile << to_string(a) + " " + to_string(b) + "\n";
    
    if( remove( "temp.txt" ) != 0 )
        perror( "Error deleting file" );

    tim = clock() - tim;
    cout << ((double)tim)/CLOCKS_PER_SEC << " seconds\n";

    TempInp.close();
    MyFile.close();

    return 0;
}