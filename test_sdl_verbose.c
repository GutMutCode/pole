#include <SDL2/SDL.h>
#include <stdio.h>

int main() {
    printf("Starting SDL test...\n");
    
    // Print SDL version
    SDL_version compiled;
    SDL_version linked;
    SDL_VERSION(&compiled);
    SDL_GetVersion(&linked);
    printf("Compiled against SDL %d.%d.%d\n", compiled.major, compiled.minor, compiled.patch);
    printf("Linked against SDL %d.%d.%d\n", linked.major, linked.minor, linked.patch);
    
    // Check available video drivers
    printf("\nAvailable video drivers:\n");
    int num_drivers = SDL_GetNumVideoDrivers();
    for (int i = 0; i < num_drivers; i++) {
        printf("  %d: %s\n", i, SDL_GetVideoDriver(i));
    }
    
    printf("\nInitializing SDL...\n");
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        printf("SDL_Init failed: %s\n", SDL_GetError());
        return 1;
    }
    
    printf("Current video driver: %s\n", SDL_GetCurrentVideoDriver());
    
    printf("\nCreating window...\n");
    SDL_Window* window = SDL_CreateWindow(
        "Test Window",
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        800, 600,
        SDL_WINDOW_SHOWN
    );
    
    if (!window) {
        printf("SDL_CreateWindow failed: %s\n", SDL_GetError());
        SDL_Quit();
        return 1;
    }
    
    printf("Window created successfully!\n");
    printf("Displaying for 5 seconds...\n");
    
    SDL_Delay(5000);
    
    printf("Cleaning up...\n");
    SDL_DestroyWindow(window);
    SDL_Quit();
    printf("Done!\n");
    
    return 0;
}
