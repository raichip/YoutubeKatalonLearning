import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

// Start Scripting
// Open Browser
WebUI.openBrowser('https://practicetestautomation.com/practice-test-login/')

WebUI.maximizeWindow()

// Set Text in Username
WebUI.setText(findTestObject('Object Repository/TC-02Script/Page_Test Login  Practice Test Automation/input_Username_username'), 
    'student')

// Set Text in Password
WebUI.setText(findTestObject('Object Repository/TC-02Script/Page_Test Login  Practice Test Automation/input_Password_password'), 
    'Password123')

// Click on Login Button
WebUI.click(findTestObject('Object Repository/TC-02Script/Page_Test Login  Practice Test Automation/button_Submit'))

// Validate the Home Page
not_run: WebUI.verifyElementPresent(findTestObject('Object Repository/TC-02Script/Page_Logged In Successfully  Practice Test Automation/h1_Logged In Successfully'), 
    0)

// Click on Logout Button
WebUI.click(findTestObject('Object Repository/TC-02Script/Page_Logged In Successfully  Practice Test Automation/a_Log out'))

// Close the browser
WebUI.closeBrowser()

