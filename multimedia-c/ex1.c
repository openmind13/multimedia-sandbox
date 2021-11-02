#include <ffmpeg/swscale.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>

AVFormatContext* pFormatContext = NULL;

int main(int argc, char* argv[]) {
    // av_register_all();

    if (avformat_open_input(&pFormatContext, argv[1], NULL, NULL) != 0) {
        return -1;
    }

    if (avformat_find_stream_info(pFormatContext, NULL) < 0) {
        return -1;
    }
}