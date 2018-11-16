#include "brute_force.h"

unsigned char digest2[MD5_DIGEST_LENGTH];
bool cracked = 0;
pthread_mutex_t plock;

struct arg_struct
{
    int n;
    size_t length;
    int start;
    int end;
    char *str;
};

int main(int argc, char *argv[])
{
    if (argc < 3)
    {
        cout << "Correct usage: brute_forcer <md5 hash> <threads>\n";
        exit(1);
    }
    else if (strlen(argv[1]) != 32 || atoi(argv[1]) < 0)
    {
        cout << "MD5 not correct length or threads < 0\n";
        exit(1);
    }
    // const char md5[]= "0fdc3cbc9a749efcaf083440a794fae4";
    char *md5 = argv[1];
    str_to_md5(md5);
    size_t length = 8;
    int num_chars = 94;
    char *str = (char *)calloc(sizeof(char), num_chars);
    gen_arr(str);
    int threads = atoi(argv[2]);
    pthread_t tid[threads];
    int task_size = num_chars / threads;
    cout << "Beginning password cracker with " << threads << " threads" << endl;
    cout << "Using MD5 hash " << md5 << endl;
    std::chrono::steady_clock::time_point start = std::chrono::steady_clock::now();
    for (int i = 0; i < threads; i++)
    {
        struct arg_struct *args = (struct arg_struct *)calloc(1, sizeof(arg_struct));
        args->str = str;
        args->n = num_chars;
        args->length = length;
        args->start = task_size * i;
        args->end = (i == threads - 1) ? num_chars : task_size * (i + 1);
        // cout << "START: " << args->start << " END: " << args->end << endl;
        pthread_create(&tid[i], NULL, thread_worker, (void *)args);
    }
    for (int i = 0; i < threads; i++)
    {
        pthread_join(tid[i], NULL);
    }
    std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
    cout << "Password found with " << threads << " threads in "
         << std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count() << "ms\n";
    return 0;
}

void *thread_worker(void *arguments)
{
    struct arg_struct *args = (struct arg_struct *)arguments;
    int start = args->start;
    int end = args->end;
    for (size_t k = 1; k < args->length; k++)
    {
        for (int i = start; i < end; i++)
        {
            string s(1, args->str[i]);
            gen_strs(args->str, s, args->n, k);
        }
    }
    return NULL;
}

// The main recursive method to print all possible strings of length "length"
void gen_strs(const char str[], string prefix, const int n, const size_t length)
{
    if (cracked)
    {
        return;
    }
    else
    {
        if (prefix.length() < length)
        {
            for (int i = 0; i < n; i++)
            {
                gen_strs(str, prefix + str[i], n, length);
            }
        }
        else if (prefix.length() == length)
        {
            unsigned char digest[MD5_DIGEST_LENGTH];
            MD5((unsigned char *)prefix.c_str(), prefix.length(), (unsigned char *)&digest);
            if (memcmp(digest, digest2, MD5_DIGEST_LENGTH) == 0)
            {
                cout << "PASSWORD: " << prefix << endl;
                cracked = 1;
                return;
            }
        }
        else
        {
            return;
        }
    }
}

void gen_arr(char arr[])
{
    for (char i = 0; i < 95; i++)
    {
        arr[(int)i] = i + 32;
    }
}

void str_to_md5(const char *str)
{
    int n = strlen(str);
    char *endptr;
    for (int i = 0; i < n; i = i + 2)
    {
        char lett[3];
        lett[2] = 0;
        strncpy(lett, str + i, 2);
        long num = strtol(lett, &endptr, 16);
        digest2[i / 2] = num;
    }
    printf("\n");
}

void md5_to_str(unsigned char *md)
{
    int i;
    for (i = 0; i < MD5_DIGEST_LENGTH; i++)
    {
        printf("%02x", md[i]);
    }
    printf("\n");
}
