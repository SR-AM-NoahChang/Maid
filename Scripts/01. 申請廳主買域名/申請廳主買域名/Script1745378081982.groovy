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
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import groovy.json.JsonSlurper
import com.kms.katalon.core.util.KeywordUtil
import java.util.Random
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper


// 函數：載入外部資源（jQuery 和 Toastr.js）
def loadExternalResources() {
	WebUI.executeJavaScript('''
        var script = document.createElement("script");
        script.src = "https://code.jquery.com/jquery-3.6.0.min.js";
        document.head.appendChild(script);
    ''', [], FailureHandling.CONTINUE_ON_FAILURE)
	WebUI.delay(1)

	WebUI.executeJavaScript('''
        var link = document.createElement("link");
        link.rel = "stylesheet";
        link.href = "https://cdnjs.cloudflare.com/ajax/libs/toastr.js/latest/toastr.min.css";
        document.head.appendChild(link);

        var script = document.createElement("script");
        script.src = "https://cdnjs.cloudflare.com/ajax/libs/toastr.js/latest/toastr.min.js";
        document.head.appendChild(script);
    ''', [], FailureHandling.CONTINUE_ON_FAILURE)
	WebUI.delay(1)

	WebUI.executeJavaScript('''
        var style = document.createElement("style");
        style.innerHTML = `
            .toast-center-top {
                top: 10% !important;
                left: 50% !important;
                transform: translate(-50%, -50%) !important;
                position: fixed !important;
                z-index: 9999;
            }
        `;
        document.head.appendChild(style);
    ''', [], FailureHandling.OPTIONAL)
}

// 顯示 loading toast
WebUI.executeJavaScript("""
    // 建立 toast 消息顯示的 HTML 元素
    var toast = document.createElement("div");
    toast.id = "dynamicToast";
    toast.style.position = "fixed";
    toast.style.top = "10%";  // 設置 toast 顯示在畫面中上
    toast.style.left = "50%";
    toast.style.transform = "translateX(-50%)";  // 水平居中
    toast.style.backgroundColor = "#333";
    toast.style.color = "#fff";
    toast.style.padding = "10px 20px";
    toast.style.borderRadius = "5px";
    toast.style.zIndex = "10000";
    toast.style.display = "flex";
    toast.style.alignItems = "center";  // 使 icon 和文字在垂直方向對齊

    // 創建 loading icon
    var loadingIcon = document.createElement("div");
    loadingIcon.style.border = "4px solid #f3f3f3";  // 淺灰色邊框
    loadingIcon.style.borderTop = "4px solid #3498db";  // 藍色上邊框
    loadingIcon.style.borderRadius = "50%";
    loadingIcon.style.width = "20px";  // 設定圓形大小
    loadingIcon.style.height = "20px";
    loadingIcon.style.animation = "spin 1s linear infinite";  // 旋轉動畫
    loadingIcon.style.marginRight = "10px";  // 在 icon 和文字之間增加 10px 的間隙

    // 創建樣式，讓 loading 圖示旋轉
    var style = document.createElement("style");
    style.innerHTML = `
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
    `;
    document.head.appendChild(style);

    // 將 loading 圖示添加到 toast 中
    toast.appendChild(loadingIcon);

    // 設置 toast 文字
    var messageText = document.createElement("span");
    messageText.innerText = "正在進行申請廳主買域名操作，請稍候...";
    toast.appendChild(messageText);

    // 添加到頁面中
    document.body.appendChild(toast);

    // 設置 toast 動畫消失效果
    setTimeout(function() {
        toast.style.transition = "opacity 1s";
        toast.style.opacity = "0";
        setTimeout(function() {
            toast.remove();
        }, 1000);
    }, 13000); // 顯示 13 秒後淡出並移除
""", [])

// 申請廳主買域名
def responsePurchaseDomain = WS.sendRequest(findTestObject('Object Repository/API/申請廳主買域名/Happy Path/申請廳主買域名'))
WS.verifyResponseStatusCode(responsePurchaseDomain, 201, FailureHandling.STOP_ON_FAILURE)

def purchase_domain = new JsonSlurper().parseText(responsePurchaseDomain.getResponseText())
GlobalVariable.PD_WORKFLOW_ID = purchase_domain.workflow_id
println("✅PD_WORKFLOW_ID: " + GlobalVariable.PD_WORKFLOW_ID)

// 取得廳主買域名詳細資料
def responseDetails = WS.sendRequest(findTestObject('Object Repository/API/申請廳主買域名/Happy Path/取得廳主買域名詳細資料'))
WS.verifyResponseStatusCode(responseDetails, 200)
def purchase_domain_detail = new JsonSlurper().parseText(responseDetails.getResponseText())
WS.verifyElementPropertyValue(responseDetails, 'workflow_id', GlobalVariable.PD_WORKFLOW_ID.toString())
WS.verifyElementPropertyValue(responseDetails, 'domain', GlobalVariable.DOMAIN.toString())
WS.verifyElementPropertyValue(responseDetails, 'workflow_name', 'CustomerApplyPurchaseDomain')
GlobalVariable.WORKFLOW_NAME = purchase_domain_detail.workflow_name

// 取得廳主買域名項目資料 (Job狀態檢查)
WebUI.delay(4)
def responseWorkflow = WS.sendRequest(findTestObject('Object Repository/API/申請廳主買域名/Happy Path/取得廳主買域名項目資料 (Job狀態檢查)'))
WS.verifyResponseStatusCode(responseWorkflow, 200)

def purchase_domain_workflow = new JsonSlurper().parseText(responseWorkflow.getResponseText())
def expectedNames = [
	"CheckDomainBlocked", "VerifyTLD", "UpdateNameServer", "UpdateDomainRecord",
	"MergeErrorRecord", "RecheckDomainResolution", "RemoveTag"
]

def actualNames = purchase_domain_workflow.collect { it.name }
expectedNames.each { expectedName ->
	assert actualNames.contains(expectedName) : "Missing expected job name: ${expectedName}"
}

def failedJobs = purchase_domain_workflow.findAll { it.status != 'success' }
if (failedJobs) {
	println "❌ 以下 job status 不為 success："
	failedJobs.each { job ->
		println "🔴 Job ID: ${job.job_id}, Name: ${job.name}, Status: ${job.status}, Message: ${job.message ?: '無訊息'}"
	}
}

purchase_domain_workflow.each { job ->
	if (expectedNames.contains(job.name)) {
		GlobalVariable."${job.name}_job_id" = job.job_id
		println "✅ 存储 job_id for ${job.name}: ${job.job_id} into GlobalVariable.${job.name}_job_id"
	}
}

// ======================
// 手動調整 Job 狀態
// ======================

// 更改 UpdateNameServer 狀態
WebUI.delay(3)
WS.sendRequest(findTestObject('Object Repository/API/申請廳主買域名/Happy Path/更改UpdateNameServer 狀態'))

// 更改 RecheckDomainResolution 狀態
WebUI.delay(3)
WS.sendRequest(findTestObject('Object Repository/API/申請廳主買域名/Happy Path/更改RecheckDomainResolution 狀態'))


