// graphics/chronometric_light.cpp
// "Chronometric Light Engine" - Synchronizes virtual sun with local time.

#include <chrono>
#include <cmath>
#include <iostream>
#include <glm/glm.hpp>

class ChronometricLight {
public:
    struct LightState {
        glm::vec3 direction;
        glm::vec3 color; // RGB
        float intensity;
        float shadow_softness;
    };

    ChronometricLight() {}

    /**
     * Calculates the sun's position and color based on the current system time.
     * @return LightState containing the calculated properties.
     */
    LightState update() {
        // 1. Get Local Time
        auto now = std::chrono::system_clock::now();
        time_t t_now = std::chrono::system_clock::to_time_t(now);
        tm* local_tm = localtime(&t_now);

        // Normalize time to 0.0 - 24.0
        float decimal_time = local_tm->tm_hour + (local_tm->tm_min / 60.0f);

        LightState state;

        // 2. Calculate Sun Position (Simple Arc)
        // Dawn at 6:00, Noon at 12:00, Dusk at 18:00
        // Angle: 6am = 0 deg, 12pm = 90 deg, 6pm = 180 deg
        float sun_angle_rad;
        
        if (decimal_time >= 6.0f && decimal_time <= 18.0f) {
            // Daytime
            float day_progress = (decimal_time - 6.0f) / 12.0f; // 0.0 to 1.0
            sun_angle_rad = day_progress * 3.14159f; // 0 to PI
            
            // X position follows the arc, Y is height
            state.direction = glm::vec3(
                cos(sun_angle_rad), // East to West
                sin(sun_angle_rad), // Height
                0.2f // Slight Z-offset for depth
            );
            state.intensity = sin(sun_angle_rad); // Brightest at noon
        } else {
            // Night Mode - Moon light (Cooler, dimmer, static)
            state.direction = glm::vec3(0.5f, 0.8f, -0.2f);
            state.intensity = 0.2f;
        }

        // 3. Calculate Color Temperature (Kelvin emulation)
        state.color = calculate_color_by_time(decimal_time);
        
        // 4. Shadow Softness based on time (Long shadows at dawn/dusk are sharper? Or softer?)
        // Let's say noon shadows are sharpest (hard overhead light), dawn/dusk are soft (atmospheric scattering).
        float closeness_to_noon = 1.0f - std::abs(decimal_time - 12.0f) / 6.0f;
        state.shadow_softness = 1.0f - std::max(0.0f, closeness_to_noon); 

        return state;
    }

private:
    glm::vec3 calculate_color_by_time(float time) {
        // RGB Values
        if (time < 6.0f || time > 18.0f) {
            return glm::vec3(0.1f, 0.1f, 0.35f); // Deep Night Blue
        } else if (time < 8.0f) {
            return glm::vec3(1.0f, 0.6f, 0.4f); // Golden Morning
        } else if (time < 16.0f) {
            return glm::vec3(0.95f, 0.98f, 1.0f); // Stark Noon White
        } else {
            return glm::vec3(1.0f, 0.5f, 0.2f); // Warm Sunset
        }
    }
};
