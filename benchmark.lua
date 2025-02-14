local base_url = "http://localhost:8080/users"

-- Request setup for simplified GET and POST methods
request = function()
    local method = math.random(1, 2)

    -- 1: GET request to retrieve a user (random ID between 1 and 100)
    if method == 1 then
        local user_id = math.random(1, 100)  -- Assuming 100 users exist
        return wrk.format("GET", base_url .. "/" .. user_id)

    -- 2: POST request to create a new user
    elseif method == 2 then
        local user_data = '{"name":"Test User ' .. math.random(1000, 9999) .. '", "email":"test' .. math.random(1000, 9999) .. '@example.com"}'
        return wrk.format("POST", base_url, nil, user_data)
    end
end
