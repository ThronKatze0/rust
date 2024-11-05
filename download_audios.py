import os
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import time
import json
import requests

IGNORE_FILES = [
    "LC 01_Mother's Little Helper (Seite 14)",
    "LC 10_Farewell to Pripchat (Seite 84)",
    "LC 32_Learning a new language (Seite 214)",
    "LC 38_Women leaders_01 (Seite 250)",
    "LC 56_Nine to Five (Seite 372)",
]

# Set up the WebDriver (use the appropriate driver for your browser)
driver = webdriver.Chrome()  # or webdriver.Firefox() for Firefox

# Replace with the URL of the page you want to scrape
driver.get('https://www.trauner-digibox.com/learning_resources?code=20k0x2knb8jf')
time.sleep(1)

def download_audio_file(url, filename='audio-file.mp3'):
    response = requests.get(url)
    with open(filename, 'wb') as file:
        file.write(response.content)

driver.switch_to.frame(driver.find_element(By.ID, 'content'))
driver.find_element(By.XPATH, '/html/body/div[2]/div/div/div[2]/div/div[2]/form/div[1]/input').send_keys("gerdaober@gmx.at")
driver.find_element(By.XPATH, '/html/body/div[2]/div/div/div[2]/div/div[2]/form/div[2]/input').send_keys("ToyotaRAV4SP236EG")
driver.find_element(By.XPATH, '/html/body/div[2]/div/div/div[2]/div/div[2]/form/input').click()
time.sleep(0.5)
# driver.find_element(By.XPATH, '/html/body/div[2]/div/div/div[1]/div[1]/div[2]/div[2]').click()
driver.get("https://www.trauner-digibox.com/learning_resources?code=20k0x2knb8jf")
time.sleep(0.2)
driver.switch_to.frame(driver.find_element(By.ID, "content"))
driver.find_element(By.XPATH, "//div[@id='category_20056']/div/img").click()

i = -1
while True:
    resource_links = driver.find_elements(By.CLASS_NAME, 'resource_link')
    print(len(resource_links))
    i += 1
    if i == len(resource_links):
        break
    resource_link = resource_links[i]
    filename = resource_link.text
    print(filename)

    if (filename in IGNORE_FILES):
        continue
    filename = filename.replace("'", "")

    resource_link.click()
    time.sleep(2)  # Wait for the content to load

    # Switch to the iframe
    driver.switch_to.frame(driver.find_element(By.ID, 'innerframe'))

    # Find the source element and get its URL
    source_element = driver.find_element(By.TAG_NAME, 'source')
    source_url = source_element.get_attribute('src')
    print(source_url)
    driver.execute_script(f"""
      const a = document.createElement('a');
      a.href = '{source_url}';
      a.download = '{filename}';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
    """)

    # Download the audio file
    # download_audio_file(source_url, filename + ".mp3")

    # Switch back to the default content


    driver.switch_to.default_content()
    time.sleep(0.2)
    driver.switch_to.frame(driver.find_element(By.ID, "content"))
    # close_button = driver.execute_script("return document.querySelectorAll('.ui-button')")
    # close_button = driver.find_element(By.XPATH, "/html/body/div[6]/div[1]/button")
    # print(close_button)
    # close_button.click()
    webdriver.ActionChains(driver).key_down(webdriver.Keys.ENTER).perform()

    while not os.path.exists(f"/home/winklerv/Downloads/{filename}.mp3"):
        print("File doesn't exist yet")
        time.sleep(0.5)
    while os.path.getsize(f"/home/winklerv/Downloads/{filename}.mp3") == 0:
        print("File downloading...")
        time.sleep(1)

# Close the WebDriver
driver.quit()
