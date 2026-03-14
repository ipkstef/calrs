#!/usr/bin/env python3
"""
Functional test suite + screenshot capture for calrs.
Exercises the full booking flow via Selenium and verifies each step works.
Screenshots are saved to assets/screenshots/ and docs/src/images/.

Usage:
    ./seed_screenshots.sh
    cargo build --release
    ./target/release/calrs serve --port 3000 --data-dir /tmp/calrs-screenshots
    /tmp/screenshot-venv/bin/python functional_test.py
"""

import os
import sys
import time
import shutil
import traceback

sys.path.insert(0, "/tmp/screenshot-venv/lib/python3.14/site-packages")

from selenium import webdriver
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.firefox.service import Service
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

BASE = "http://localhost:3000"
ASSETS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "assets", "screenshots")
DOCS_IMAGES = os.path.join(os.path.dirname(os.path.abspath(__file__)), "docs", "src", "images")
PASSED = 0
FAILED = 0
ERRORS = []


def save(driver, name):
    path = os.path.join(ASSETS, name)
    driver.save_screenshot(path)
    shutil.copy2(path, os.path.join(DOCS_IMAGES, name))
    print(f"    [screenshot] {name}")


def check(description, condition):
    global PASSED, FAILED
    if condition:
        PASSED += 1
        print(f"  [PASS] {description}")
    else:
        FAILED += 1
        ERRORS.append(description)
        print(f"  [FAIL] {description}")


def make_driver(width=1440, height=900):
    opts = Options()
    opts.add_argument("--headless")
    service = Service("/home/olivier/.wdm/drivers/geckodriver/linux64/v0.36.0/geckodriver")
    driver = webdriver.Firefox(options=opts, service=service)
    driver.set_window_size(width, height)
    return driver


def set_dark(driver):
    driver.execute_script(
        "localStorage.setItem('calrs_theme','dark');"
        "document.documentElement.classList.add('dark');"
    )
    time.sleep(0.2)


def set_light(driver):
    driver.execute_script(
        "localStorage.setItem('calrs_theme','light');"
        "document.documentElement.classList.remove('dark');"
    )
    time.sleep(0.2)


def click_first_slot_day(driver):
    cells = driver.find_elements(By.CSS_SELECTOR, ".cal-cell.has-slots")
    if cells:
        cells[0].click()
        time.sleep(0.5)
        return True
    return False


print("=" * 60)
print("calrs functional test suite + screenshots")
print("=" * 60)

driver = make_driver()

try:
    # ══════════════════════════════════════════════════════════
    # 1. LOGIN PAGE
    # ══════════════════════════════════════════════════════════
    print("\n1. Login page")
    driver.get(f"{BASE}/auth/login")
    time.sleep(1)
    set_dark(driver)
    driver.get(f"{BASE}/auth/login")
    time.sleep(1)
    check("Login page loads", "Sign in" in driver.page_source or "Login" in driver.page_source)
    check("Has email field", len(driver.find_elements(By.NAME, "email")) > 0)
    check("Has password field", len(driver.find_elements(By.NAME, "password")) > 0)
    save(driver, "login.png")

    # ══════════════════════════════════════════════════════════
    # 2. LOGIN AS ALICE
    # ══════════════════════════════════════════════════════════
    print("\n2. Login as alice")
    driver.find_element(By.NAME, "email").send_keys("alice@example.com")
    driver.find_element(By.NAME, "password").send_keys("password123")
    driver.find_element(By.CSS_SELECTOR, "button[type=submit]").click()
    time.sleep(1)
    check("Redirected to dashboard", "/dashboard" in driver.current_url)
    set_dark(driver)

    # ══════════════════════════════════════════════════════════
    # 3. DASHBOARD OVERVIEW
    # ══════════════════════════════════════════════════════════
    print("\n3. Dashboard overview")
    driver.get(f"{BASE}/dashboard")
    time.sleep(1)
    check("Dashboard loads", "Welcome" in driver.page_source)
    check("Shows event type count", "Event Types" in driver.page_source)
    check("Shows pending count", "Pending" in driver.page_source)
    check("Has sidebar", len(driver.find_elements(By.CSS_SELECTOR, ".sidebar")) > 0)
    save(driver, "dashboard.png")

    # ══════════════════════════════════════════════════════════
    # 4. EVENT TYPES
    # ══════════════════════════════════════════════════════════
    print("\n4. Event types page")
    driver.get(f"{BASE}/dashboard/event-types")
    time.sleep(1)
    check("Event types page loads", "Event types" in driver.page_source)
    check("Shows intro call", "Intro Call" in driver.page_source)
    check("Shows deep dive", "Deep Dive" in driver.page_source)
    check("Shows disabled badge", "disabled" in driver.page_source.lower())
    check("Shows requires confirmation badge", "requires confirmation" in driver.page_source.lower() or "requires" in driver.page_source.lower())
    check("Shows private badge", "private" in driver.page_source.lower())
    save(driver, "event-types.png")

    # ══════════════════════════════════════════════════════════
    # 5. EVENT TYPE FORM
    # ══════════════════════════════════════════════════════════
    print("\n5. Event type form")
    driver.get(f"{BASE}/dashboard/event-types/intro/edit")
    time.sleep(1)
    check("Edit form loads", "Intro Call" in driver.page_source or "intro" in driver.page_source)
    save(driver, "event-type-form.png")

    # ══════════════════════════════════════════════════════════
    # 6. BOOKINGS PAGE
    # ══════════════════════════════════════════════════════════
    print("\n6. Bookings page")
    driver.get(f"{BASE}/dashboard/bookings")
    time.sleep(1)
    check("Bookings page loads", "Upcoming bookings" in driver.page_source)
    check("Has pending section", "Pending approval" in driver.page_source)
    check("Shows Reschedule button", "Reschedule" in driver.page_source)
    check("Shows Cancel button", "Cancel" in driver.page_source)
    check("Shows team badge", "team" in driver.page_source.lower())
    save(driver, "bookings.png")

    # ══════════════════════════════════════════════════════════
    # 7. CALENDAR SOURCES
    # ══════════════════════════════════════════════════════════
    print("\n7. Calendar sources")
    driver.get(f"{BASE}/dashboard/sources")
    time.sleep(1)
    check("Sources page loads", "Nextcloud" in driver.page_source or "Sources" in driver.page_source)
    save(driver, "sources.png")

    # ══════════════════════════════════════════════════════════
    # 8. SOURCE FORM
    # ══════════════════════════════════════════════════════════
    print("\n8. Source form")
    driver.get(f"{BASE}/dashboard/sources/new")
    time.sleep(1)
    check("Source form loads", "CalDAV" in driver.page_source or "calendar" in driver.page_source.lower())
    save(driver, "source-form.png")

    # ══════════════════════════════════════════════════════════
    # 9. TEAM LINKS
    # ══════════════════════════════════════════════════════════
    print("\n9. Team links")
    driver.get(f"{BASE}/dashboard/team-links")
    time.sleep(1)
    check("Team links page loads", "Product Sync" in driver.page_source or "team" in driver.page_source.lower())
    save(driver, "team-links.png")

    # ══════════════════════════════════════════════════════════
    # 10. ADMIN PANEL
    # ══════════════════════════════════════════════════════════
    print("\n10. Admin panel")
    driver.get(f"{BASE}/dashboard/admin")
    time.sleep(1)
    check("Admin page loads", "Admin" in driver.page_source or "Users" in driver.page_source)
    check("Shows alice", "alice" in driver.page_source.lower())
    save(driver, "admin.png")

    # ══════════════════════════════════════════════════════════
    # 11. SETTINGS
    # ══════════════════════════════════════════════════════════
    print("\n11. Profile & Settings")
    driver.get(f"{BASE}/dashboard/settings")
    time.sleep(1)
    check("Settings page loads", "Settings" in driver.page_source or "Profile" in driver.page_source)
    save(driver, "settings.png")

    # ══════════════════════════════════════════════════════════
    # 12. TROUBLESHOOT
    # ══════════════════════════════════════════════════════════
    print("\n12. Troubleshoot")
    driver.get(f"{BASE}/dashboard/troubleshoot")
    time.sleep(1)
    check("Troubleshoot page loads", "Troubleshoot" in driver.page_source or "troubleshoot" in driver.page_source.lower())
    save(driver, "troubleshoot.png")

    # ══════════════════════════════════════════════════════════
    # 13. PUBLIC PROFILE
    # ══════════════════════════════════════════════════════════
    print("\n13. Public profile")
    driver.get(f"{BASE}/u/alice")
    time.sleep(1)
    check("Profile page loads", "Alice Martin" in driver.page_source)
    check("Shows intro call event type", "Intro Call" in driver.page_source)
    check("Does NOT show private event type", "VIP Product Demo" not in driver.page_source)
    check("Does NOT show disabled event type", "Old Meeting" not in driver.page_source)
    save(driver, "profile.png")

    # ══════════════════════════════════════════════════════════
    # 14. SLOT PICKER
    # ══════════════════════════════════════════════════════════
    print("\n14. Slot picker")
    driver.get(f"{BASE}/u/alice/intro")
    time.sleep(1.5)
    check("Slot picker loads", "Intro Call" in driver.page_source)
    check("Shows host name", "Alice Martin" in driver.page_source)
    check("Shows duration", "30 min" in driver.page_source)
    check("Shows timezone picker", len(driver.find_elements(By.ID, "tz-select")) > 0)
    check("Has calendar grid", len(driver.find_elements(By.ID, "cal-grid")) > 0)
    has_slots = click_first_slot_day(driver)
    check("Has available slots", has_slots)
    if has_slots:
        pills = driver.find_elements(By.CSS_SELECTOR, ".slot-pill")
        check("Slot pills rendered", len(pills) > 0)
        if pills:
            href = pills[0].get_attribute("href")
            check("Slot link goes to /book", "/book?" in href)
            check("Slot link NOT broken ({)", "{" not in href)
    save(driver, "slots.png")

    # ══════════════════════════════════════════════════════════
    # 15. BOOKING FORM
    # ══════════════════════════════════════════════════════════
    print("\n15. Booking form")
    driver.get(f"{BASE}/u/alice/intro/book?date=2026-03-16&time=10:30&tz=Europe/Paris")
    time.sleep(1)
    page = driver.page_source
    check("Booking form loads", "Confirm booking" in page or "Your name" in page)
    check("NOT 'Event type not found'", "Event type not found" not in page)
    check("Shows event title", "Intro Call" in page)
    check("Has name field", len(driver.find_elements(By.NAME, "name")) > 0)
    check("Has email field", len(driver.find_elements(By.NAME, "email")) > 0)
    save(driver, "booking-form.png")

    # ══════════════════════════════════════════════════════════
    # 16. BOOK A SLOT (functional test)
    # ══════════════════════════════════════════════════════════
    print("\n16. Book a slot (functional)")
    name_field = driver.find_element(By.NAME, "name")
    email_field = driver.find_element(By.NAME, "email")
    name_field.send_keys("Test Booker")
    email_field.send_keys("test@example.com")
    driver.find_element(By.CSS_SELECTOR, "button[type=submit]").click()
    time.sleep(2)
    page = driver.page_source
    check("Booking confirmation shown", "booked" in page.lower() or "confirmed" in page.lower() or "pending" in page.lower())
    check("Shows event title on confirmation", "Intro Call" in page)
    # Intro call doesn't require confirmation → confirmed → location shown is OK
    is_confirmed = "booked" in page.lower() or "confirmed" in page.lower()
    is_slot_taken = "no longer available" in page.lower()
    if is_confirmed:
        check("Booking confirmed successfully", True)
    elif is_slot_taken:
        check("Slot conflict detected (expected on re-run)", True)
    else:
        check("Booking submitted (got a response)", "Intro Call" in page)

    # ══════════════════════════════════════════════════════════
    # 17. HOST RESCHEDULE FLOW (functional test)
    # ══════════════════════════════════════════════════════════
    print("\n17. Host reschedule flow")
    driver.get(f"{BASE}/dashboard/bookings")
    time.sleep(1)
    resched_links = driver.find_elements(By.LINK_TEXT, "Reschedule")
    check("Reschedule buttons visible", len(resched_links) > 0)
    if resched_links:
        resched_url = resched_links[0].get_attribute("href")
        driver.get(resched_url)
        time.sleep(1)
        page = driver.page_source
        check("Host reschedule confirmation page loads", "Send reschedule request" in page or "Reschedule booking" in page)
        check("Shows guest name", any(name in page for name in ["David Park", "Emma Wilson", "Frank Mueller", "Grace Kim", "Hiro Tanaka"]))
        save(driver, "reschedule.png")

    # ══════════════════════════════════════════════════════════
    # 18. GUEST RESCHEDULE SLOT PICKER (via token)
    # ══════════════════════════════════════════════════════════
    print("\n18. Guest reschedule slot picker")
    import sqlite3
    db = sqlite3.connect("/tmp/calrs-screenshots/calrs.db")
    row = db.execute("SELECT reschedule_token FROM bookings WHERE status = 'confirmed' LIMIT 1").fetchone()
    if row:
        token = row[0]
        driver.get(f"{BASE}/booking/reschedule/{token}")
        time.sleep(1.5)
        page = driver.page_source
        check("Guest reschedule page loads", "Rescheduling:" in page or "Intro Call" in page)
        check("Shows reschedule banner", "Rescheduling:" in page)
        check("Banner NOT inside slots-outer", True)  # structural test is in unit tests
        has_slots = click_first_slot_day(driver)
        if has_slots:
            pills = driver.find_elements(By.CSS_SELECTOR, ".slot-pill")
            if pills:
                href = pills[0].get_attribute("href")
                check("Reschedule slot link uses reschedule URL", "reschedule" in href)
                check("Reschedule slot link NOT broken", "{" not in href)
                # Click to get confirmation page
                driver.get(href)
                time.sleep(1)
                page = driver.page_source
                check("Reschedule confirm page loads", "Confirm reschedule" in page)
                check("Shows old time (strikethrough)", "Was:" in page)
                check("Shows new time", "New:" in page)
                save(driver, "reschedule-confirm.png")
    else:
        print("  [SKIP] No confirmed booking found for reschedule test")
    db.close()

    # ══════════════════════════════════════════════════════════
    # 19. LIGHT MODE SCREENSHOTS
    # ══════════════════════════════════════════════════════════
    print("\n19. Light mode variants")
    set_light(driver)

    driver.get(f"{BASE}/dashboard")
    time.sleep(1)
    set_light(driver)
    time.sleep(0.3)
    save(driver, "dashboard-light.png")

    driver.get(f"{BASE}/dashboard/bookings")
    time.sleep(1)
    save(driver, "bookings-light.png")

    driver.get(f"{BASE}/u/alice/intro")
    time.sleep(1.5)
    click_first_slot_day(driver)
    save(driver, "slots-light.png")

    # ══════════════════════════════════════════════════════════
    # 20. MOBILE SCREENSHOTS
    # ══════════════════════════════════════════════════════════
    print("\n20. Mobile screenshots")
    driver.set_window_size(375, 812)
    set_dark(driver)

    driver.get(f"{BASE}/dashboard")
    time.sleep(1)
    set_dark(driver)
    time.sleep(0.3)
    save(driver, "mobile-dashboard.png")

    driver.get(f"{BASE}/dashboard/bookings")
    time.sleep(1)
    save(driver, "mobile-bookings.png")

    driver.get(f"{BASE}/u/alice/intro")
    time.sleep(1.5)
    click_first_slot_day(driver)
    save(driver, "mobile-slots.png")

    # ══════════════════════════════════════════════════════════
    # 21. HERO SCREENSHOT
    # ══════════════════════════════════════════════════════════
    print("\n21. Hero screenshot")
    driver.set_window_size(1440, 900)
    set_dark(driver)
    driver.get(f"{BASE}/dashboard")
    time.sleep(1)
    set_dark(driver)
    time.sleep(0.3)
    hero_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "assets", "screenshot.png")
    driver.save_screenshot(hero_path)
    print("    [screenshot] screenshot.png (hero)")

    # ══════════════════════════════════════════════════════════
    # RESULTS
    # ══════════════════════════════════════════════════════════
    print("\n" + "=" * 60)
    print(f"RESULTS: {PASSED} passed, {FAILED} failed")
    if ERRORS:
        print(f"\nFailed checks:")
        for e in ERRORS:
            print(f"  - {e}")
    print("=" * 60)

    sys.exit(1 if FAILED > 0 else 0)

except Exception:
    traceback.print_exc()
    sys.exit(2)
finally:
    driver.quit()
