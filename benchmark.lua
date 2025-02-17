local base_url = "http://localhost:8081"

local first_names = {"Emma", "Liam", "Olivia", "Noah", "Ava", "Oliver", "Isabella", "Lucas", "Sophia", "Mason"}
local last_names = {"Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis", "Rodriguez", "Martinez"}
local domains = {"gmail.com", "yahoo.com", "hotmail.com", "outlook.com", "example.com"}

local request_counter = 0
local success_counter = 0
local error_counter = 0
local start_time = 0

local function generate_user_data()
    local first_name = first_names[math.random(#first_names)]
    local last_name = last_names[math.random(#last_names)]
    local domain = domains[math.random(#domains)]
    
    request_counter = request_counter + 1
    local email = string.format("%s.%s.%d@%s", string.lower(first_name), string.lower(last_name), request_counter, domain)
    
    return string.format('{"name":"%s %s", "email":"%s"}', first_name, last_name, email)
end

function init(args)
    start_time = os.time()
    math.randomseed(os.time())
end

function request()
    local method = math.random(1, 100)
    local headers = { ["Content-Type"] = "application/json", ["Accept"] = "application/json" }
    
    if method <= 40 then
        return wrk.format("GET", base_url, headers)
    
    elseif method <= 80 then
        local payload = generate_user_data()
        headers["Content-Length"] = #payload
        return wrk.format("POST", base_url .. "/create", headers, payload)
    
    elseif method <= 90 then
        local user_id = math.random(1, 100)
        local payload = generate_user_data()
        headers["Content-Length"] = #payload
        return wrk.format("PUT", base_url .. "/" .. user_id .. "/update", headers, payload)
    
    else
        local user_id = math.random(1, 100)
        return wrk.format("DELETE", base_url .. "/" .. user_id .. "/delete", headers)
    end
end

function response(status, headers, body)
    if status == 200 or status == 201 then
        success_counter = success_counter + 1
    else
        error_counter = error_counter + 1
    end
end

function done(summary, latency, requests)
    local duration = os.time() - start_time
    local rps = duration > 0 and request_counter/duration or 0
    
    print("\n=== Test Results ===")
    print(string.format("Duration: %d seconds", duration))
    print(string.format("Total Requests: %d", request_counter))
    print(string.format("Successful Requests: %d", success_counter))
    print(string.format("Failed Requests: %d", error_counter))
    print(string.format("Requests/sec: %.2f", rps))
    print(string.format("Average Latency: %.2fms", latency.mean/1000))
    print(string.format("Max Latency: %.2fms", latency.max/1000))
end
