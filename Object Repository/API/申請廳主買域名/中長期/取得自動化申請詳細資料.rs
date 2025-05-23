<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>取得自動化申請詳細資料</name>
   <tag></tag>
   <elementGuidId>c9c44a1a-96bb-4d6e-91fd-6b082645fd21</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>88d52d25-9039-431d-b980-6073ee4861d1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-API-Key</name>
      <type>Main</type>
      <value>${ADM_KEY}</value>
      <webElementGuid>d0898856-872f-40ca-9f0b-a223ad09fe61</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d8085c0e-0c37-4fd7-b3a4-25ffdb776cb4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.7.4</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${DEV}/workflow_api/adm/workflows/${WORKFLOW_ID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.DEV</defaultValue>
      <description></description>
      <id>8d20b491-3ba9-4560-8570-2ff69cd3650c</id>
      <masked>false</masked>
      <name>DEV</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.WORKFLOW_ID</defaultValue>
      <description></description>
      <id>e828af70-56ae-4e6d-99de-94a5de9ad4e3</id>
      <masked>false</masked>
      <name>WORKFLOW_ID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ADM_KEY</defaultValue>
      <description></description>
      <id>cab9183f-91b3-4367-8791-47eaa97b9eb9</id>
      <masked>false</masked>
      <name>ADM_KEY</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
