// userspace/system_services/context_orb/main.cpp
// The Cognitive Kernel Interface: Context Orb
// Handles Drag & Drop intents and interfaces with the Local LLM.

#include <iostream>
#include <string>
#include <vector>
#include <filesystem>
#include <thread>
#include <chrono>

// Mock dependencies for the architectural prototype
namespace mock {
    struct Json {
        std::string payload;
        static Json object() { return {"{}"}; }
        void add(std::string key, std::string val) { 
            payload += "\"" + key + "\": \"" + val + "\", "; 
        }
        std::string dump() { return payload; }
    };

    struct LLMResponse {
        std::vector<std::string> suggested_actions;
    };

    // Simulates the Local LLM API
    LLMResponse query_llm(const std::string& context_prompt) {
        std::cout << "[CognitiveKernel] Querying LLM with: " << context_prompt << std::endl;
        // In a real system, this would curl localhost:8080/v1/completions
        
        // Hardcoded mock logic for prototype
        if (context_prompt.find("PDF") != std::string::npos) {
            return {{"Summarize", "Convert to Word", "Email to Finance"}};
        } else if (context_prompt.find("Image") != std::string::npos) {
            return {{"Remove Background", "Upscale 4K", "Extract Text (OCR)"}};
        } else {
            return {{"Copy", "Share", "Delete"}};
        }
    }
}

class MimeDetector {
public:
    static std::string detect(const std::filesystem::path& path) {
        std::string ext = path.extension().string();
        if (ext == ".pdf") return "application/pdf";
        if (ext == ".png" || ext == ".jpg") return "image/png";
        if (ext == ".txt" || ext == ".md") return "text/plain";
        return "application/octet-stream";
    }
};

class ContextOrb {
public:
    void run() {
        std::cout << "[ContextOrb] Service Started. Hovering at (50%, 90%)..." << std::endl;
        
        // Main Event Loop
        while (true) {
            // 1. Listen for Global Mouse Events (Mocked input loop)
            simulate_event_loop();
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
        }
    }

private:
    void simulate_event_loop() {
        // In a real scenario, we read from /dev/aura_input or X11/Wayland socket
        // Here we simulate a drop event every 5 seconds for demonstration.
        static int tick = 0;
        tick++;
        
        if (tick == 20) { // Approx 2 seconds
            handle_drop_event("/home/user/downloads/invoice_2025.pdf");
        }
        if (tick == 50) { // Approx 5 seconds
            handle_drop_event("/home/user/photos/vacation.png");
            tick = 0;
        }
    }

    void handle_drop_event(const std::filesystem::path& path) {
        std::cout << "\n[ContextOrb] DROP DETECTED: " << path << std::endl;

        // 2. Identify File Type
        std::string mime = MimeDetector::detect(path);
        std::cout << "[ContextOrb] MIME Type: " << mime << std::endl;

        // 3. Construct Prompt for LAM (Large Action Model)
        std::string prompt = "User dropped file of type " + mime + 
                           " named " + path.filename().string() + 
                           ". Context: Work Workspace. Suggest actions.";

        // 4. Query Cognitive Kernel
        auto response = mock::query_llm(prompt);

        // 5. Output JSON for UI Compositor to render "Glass Bubbles"
        emit_ui_update(response.suggested_actions);
    }

    void emit_ui_update(const std::vector<std::string>& actions) {
        std::cout << "[ContextOrb] Sending UI Update to Compositor:" << std::endl;
        std::cout << "  {" << std::endl;
        std::cout << "    \"type\": \"intent_bubbles\"," << std::endl;
        std::cout << "    \"actions\": [";
        for (size_t i = 0; i < actions.size(); ++i) {
            std::cout << "\"" << actions[i] << "\"" << (i < actions.size() - 1 ? ", " : "");
        }
        std::cout << "]" << std::endl;
        std::cout << "  }" << std::endl;
    }
};

int main() {
    std::cout << "=======================================" << std::endl;
    std::cout << "   AURA OS - COGNITIVE KERNEL v0.1     " << std::endl;
    std::cout << "=======================================" << std::endl;

    ContextOrb orb;
    orb.run();

    return 0;
}
