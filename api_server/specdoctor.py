import json, random


def pas(spec):
    oment = ""
    cment = ""
    rment = ""
    vment = ""

    if spec["OS"] == "Arch Linux":
        oment = "역시 아치가 최고죠, 안그런가요?"
    if spec["OS"] == "Windows":
        oment = "아마 이 프로그램을 쓰는 사람 대부분이 윈도울를 쓸거에요"
    if spec["OS"] == "Ubuntu":
        oment = "입문하기 좋은 리눅스에요, 그만큼 규모도 크죠."

    if spec["CPU_len"] == 1:
        cment = "아마도 가상머신이거나, 전설이거나겠죠"
    if spec["CPU_len"] in range(2, 6):
        cment = (
            "SBC, 개발보드거나 절약왕이네요, 게임을 즐기지 않는다면 충분하다고 생각해요"
        )
    if spec["CPU_len"] in range(6, 9):
        cment = (
            "적어도 당분간 cpu를 바꿀일은 없겠네요, 이런저런 요인도 생각해야하지만요"
        )
    if spec["CPU_len"] in range(9, 17):
        cment = "이 정도면 개인이 소지하기엔 부족합이 없겠네요"
    if spec["CPU_len"] > 90:
        cment = (
            "쓰레드리퍼...? 아니면 벌써 뜯은사람이 나온건가요? 어느쪽이든 대단하네요"
        )
    print(
        f"""
            OS는 {spec["OS"]}를 사용중이에요. {oment}
            CPU는 {spec["CPU_len"]} 개 정도네요{cment}
            메모리는 현재: {spec["mem_used"]} 총: {spec["mem_total"]} 가상메모리는 {spec["vmem_total"]} 면
        """
    )
