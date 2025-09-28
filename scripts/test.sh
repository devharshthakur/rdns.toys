#!/bin/bash

# DNS Toys Testing Menu Script
# Interactive menu to test different DNS services
# Made by AI Assistant

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
DNS_SERVER="127.0.0.1"
DNS_PORT="8053"
DOMAIN="localhost"

# Function to print colored output
print_color() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to test a DNS query
test_dns() {
    local query=$1
    local record_type=${2:-"TXT"}
    local description=$3
    
    print_color $CYAN "Testing: $description"
    print_color $YELLOW "Query: $query"
    print_color $BLUE "Command: dig @$DNS_SERVER -p $DNS_PORT $query $record_type"
    echo "----------------------------------------"
    
    dig @$DNS_SERVER -p $DNS_PORT $query $record_type
    echo ""
    echo "Press Enter to continue..."
    print_color $PURPLE "To exit the test menu at any time, press Ctrl+C."
    read
}

# Function to show help
show_help() {
    print_color $GREEN "Available DNS Services:"
    echo "1. IP Service - Get your IP address"
    echo "2. UUID Service - Generate UUIDs"
    echo "3. Pi Service - Get Pi constant"
    echo "4. Geo Service - Get location information"
    echo "5. Random Service - Generate random numbers"
    echo "6. Help Service - Show available services"
    echo "7. Test All Services"
    echo "8. Custom Query"
    echo "9. Exit"
    echo ""
}

# Function to test IP service
test_ip_service() {
    print_color $GREEN "=== IP Service Tests ==="
    test_dns "ip.$DOMAIN" "TXT" "IP as TXT record"
    test_dns "ip.$DOMAIN" "A" "IP as A record"
}

# Function to test UUID service
test_uuid_service() {
    print_color $GREEN "=== UUID Service Tests ==="
    test_dns "1.uuid.$DOMAIN" "TXT" "Single UUID"
    test_dns "5.uuid.$DOMAIN" "TXT" "5 UUIDs"
    test_dns "10.uuid.$DOMAIN" "TXT" "10 UUIDs"
}

# Function to test Pi service
test_pi_service() {
    print_color $GREEN "=== Pi Service Tests ==="
    test_dns "pi.$DOMAIN" "TXT" "Pi as text"
    test_dns "pi.$DOMAIN" "A" "Pi as IPv4 address"
    test_dns "pi.$DOMAIN" "AAAA" "Pi as IPv6 address"
}

# Function to test Geo service
test_geo_service() {
    print_color $GREEN "=== Geo Service Tests ==="
    test_dns "mumbai.geo.$DOMAIN" "TXT" "Mumbai location info"
    test_dns "london.geo.$DOMAIN" "TXT" "London location info"
    test_dns "new york.geo.$DOMAIN" "TXT" "New York location info"
    test_dns "tokyo.geo.$DOMAIN" "TXT" "Tokyo location info"
    test_dns "london/uk.geo.$DOMAIN" "TXT" "London, UK specifically"
}

# Function to test Random service
test_random_service() {
    print_color $GREEN "=== Random Service Tests ==="
    test_dns "1-100.random.$DOMAIN" "TXT" "Random number 1-100"
    test_dns "1-10.random.$DOMAIN" "TXT" "Random number 1-10"
    test_dns "50-100.random.$DOMAIN" "TXT" "Random number 50-100"
}

# Function to test Help service
test_help_service() {
    print_color $GREEN "=== Help Service Tests ==="
    test_dns "help.$DOMAIN" "TXT" "Show available services"
}

# Function to test all services
test_all_services() {
    print_color $PURPLE "=== Testing All Services ==="
    test_ip_service
    test_uuid_service
    test_pi_service
    test_geo_service
    test_random_service
    test_help_service
}


# Function to check if DNS server is running
check_dns_server() {
    print_color $YELLOW "Checking if DNS server is running..."
    if dig @$DNS_SERVER -p $DNS_PORT help.$DOMAIN TXT +timeout=2 +tries=1 >/dev/null 2>&1; then
        print_color $GREEN "✅ DNS server is running on $DNS_SERVER:$DNS_PORT"
        return 0
    else
        print_color $RED "❌ DNS server is not running on $DNS_SERVER:$DNS_PORT"
        print_color $YELLOW "Please start the server with: just run"
        return 1
    fi
}

# Main menu loop
main_menu() {
    while true; do
        clear
        print_color $PURPLE "=========================================="
        print_color $PURPLE "        DNS Toys Testing Menu"
        print_color $PURPLE "=========================================="
        echo ""
        
        show_help
        
        read -p "Choose an option (1-9): " choice
        
        case $choice in
            1)
                test_ip_service
                ;;
            2)
                test_uuid_service
                ;;
            3)
                test_pi_service
                ;;
            4)
                test_geo_service
                ;;
            5)
                test_random_service
                ;;
            6)
                test_help_service
                ;;
            7)
                test_all_services
                ;;
            8)
                print_color $GREEN "Goodbye! 👋"
                exit 0
                ;;
            *)
                print_color $RED "Invalid option. Please choose 1-9."
                echo "Press Enter to continue..."
                read
                ;;
        esac
    done
}

# Main execution
main() {
    print_color $BLUE "DNS Toys Testing Menu"
    print_color $BLUE "===================="
    echo ""
    
    # Check if DNS server is running
    if ! check_dns_server; then
        exit 1
    fi
    
    echo ""
    print_color $YELLOW "Press Enter to start the menu..."
    read
    
    main_menu
}

# Run the main function
main
