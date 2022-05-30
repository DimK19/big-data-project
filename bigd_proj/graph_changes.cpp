#include <iostream>
#include <stdlib.h>
#include <math.h>
#include <fstream>
#include <time.h>


using namespace std;

int main(int argc, char **argv){

    if(argc != 2){
        cout << "You need to give parameter for percentage of changes(0-100)\n";
        return 0;
    }

    ofstream MyFile("edges_to_change.txt");
    ifstream MyInput("random_graph.txt");
    ofstream TempFile("temp.txt");
    
    time_t t;
    srand((unsigned) time(&t));

    double prob = (double) atoi(argv[1]) / 100; // argv[1] deixnei to pososto twn allagwn sto round

    clock_t tim;
    tim = clock();

    int a,b;
    int count = 0;

    while (MyInput >> a >> b){
        int temp = rand() % 100;
        if(temp < (int)(prob*100)){
            count++;
            TempFile << to_string(a) + " " + to_string(b) + "\n";
        }
    }

    MyInput.close();
    TempFile.close();

    ifstream TempInp("temp.txt");

    MyFile << count << "\n";

    while (TempInp >> a >> b)
        MyFile << to_string(a) + " " + to_string(b) + " -1\n";
    
    if( remove( "temp.txt" ) != 0 )
        perror( "Error deleting file" );

    tim = clock() - tim;
    cout << ((double)tim)/CLOCKS_PER_SEC << " seconds\n";



    return 0;
}